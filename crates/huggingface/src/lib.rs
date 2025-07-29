use std::path::PathBuf;

/// A Huggingface repository in the form huggingface.co/<repo>@<revision>:<quantization>.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HuggingfaceRepo {
    /// The repository name, for example "meta-llama/Llama-4-Scout-17B-16E-Instruct".
    pub repo: String,
    /// The revision of the repository, for example "main" or a specific commit hash.
    pub revision: Option<String>,
    /// The quantization type, for example "Q4_K_M" or "Q8_0".
    pub quantization: Option<String>,
}

impl std::str::FromStr for HuggingfaceRepo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(suffix) = s.strip_prefix("huggingface.co/") {
            let (prefix, quantization) = suffix
                .split_once(':')
                .map(|(repo, quantization)| (repo, Some(quantization.to_string())))
                .unwrap_or((suffix, None));
            let (repo, version) = prefix
                .split_once('@')
                .map(|(prefix, version)| (prefix, Some(version.to_string())))
                .unwrap_or((prefix, None));
            Ok(HuggingfaceRepo {
                repo: repo.to_string(),
                revision: version,
                quantization,
            })
        } else {
            anyhow::bail!(
                "Invalid Huggingface repository - expected huggingface.co/<repo>@[revision]:[quantization], got: {}",
                s
            );
        }
    }
}

impl std::fmt::Display for HuggingfaceRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "huggingface.co/{}", self.repo)?;
        if let Some(revision) = &self.revision {
            write!(f, "@{revision}")?;
        }
        if let Some(quantization) = &self.quantization {
            write!(f, ":{quantization}")?;
        }

        Ok(())
    }
}

/// Interface for Hugging Face repository.
pub struct RepositoryInterface {
    /// The Hugging Face API client.
    api_client: hf_hub::api::tokio::Api,
    /// The Hugging Face cache.
    cache: hf_hub::Cache,
}

/// The type of model storage.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ModelStorageType {
    /// GGUF format.
    GGUF,
    /// ONNX format.
    ONNX,
    /// Safetensors format.
    Safetensors,
    /// TensorRT format.
    TensorRT,
}

#[derive(Debug)]
/// Information about a Hugging Face repository.
pub struct RepositoryInfo {
    /// The repository API.
    repository: hf_hub::api::tokio::ApiRepo,
    /// The model cache.
    model_cache: hf_hub::CacheRepo,
    /// The top level files in the repository.
    pub files: Vec<String>,
}

impl RepositoryInfo {
    /// Download files.
    pub async fn download(
        &self,
        files: &[&str],
    ) -> Result<Vec<std::path::PathBuf>, hf_hub::api::tokio::ApiError> {
        let mut file_download_coroutines = Vec::with_capacity(files.len());
        for file in files {
            file_download_coroutines.push(self.repository.get(file));
        }

        futures::future::try_join_all(file_download_coroutines.into_iter()).await
    }

    /// Get the directory the repository is downloaded to.
    pub fn get_path(&self, filename: &str) -> Option<PathBuf> {
        self.model_cache.get(filename)
    }
}

impl RepositoryInfo {
    /// Whether the repository has GGUF files.
    pub fn has_gguf_files(&self) -> bool {
        self.files.iter().any(|f| f.ends_with(".gguf"))
    }

    /// Whether the repository has ONNX files.
    pub fn has_onnx_files(&self) -> bool {
        self.files.iter().any(|f| f.ends_with(".onnx"))
    }

    /// Whether the repository has Safetensor files.
    pub fn has_safetensor_files(&self) -> bool {
        self.files.iter().any(|f| f.ends_with(".safetensors"))
    }

    /// Whether the repository has TensorRT files.
    pub fn has_tensorrt_files(&self) -> bool {
        self.files.iter().any(|f| f.ends_with(".engine"))
    }

    pub fn get_model_storage_type(&self) -> Option<ModelStorageType> {
        if self.has_onnx_files() {
            Some(ModelStorageType::ONNX)
        } else if self.has_gguf_files() {
            Some(ModelStorageType::GGUF)
        } else if self.has_tensorrt_files() {
            Some(ModelStorageType::TensorRT)
        } else if self.has_safetensor_files() {
            Some(ModelStorageType::Safetensors)
        } else {
            None
        }
    }
}

impl RepositoryInterface {
    pub fn new() -> anyhow::Result<Self> {
        let client = hf_hub::api::tokio::ApiBuilder::from_env().high().build()?;
        let cache = hf_hub::Cache::from_env();
        Ok(Self {
            api_client: client,
            cache,
        })
    }

    pub async fn get_repository_metadata(
        &self,
        repository: &HuggingfaceRepo,
    ) -> anyhow::Result<RepositoryInfo> {
        let repository = match &repository.revision {
            Some(revision) => hf_hub::Repo::with_revision(
                repository.repo.clone(),
                hf_hub::RepoType::Model,
                revision.clone(),
            ),
            None => hf_hub::Repo::model(repository.repo.clone()),
        };

        let model = self.api_client.repo(repository.clone());
        let model_cache = self.cache.repo(repository);
        let info = model.info().await?;

        Ok(RepositoryInfo {
            repository: model,
            model_cache,
            files: info.siblings.into_iter().map(|f| f.rfilename).collect(),
        })
    }
}

/// Download an ONNX file from a Hugging Face repository.
pub async fn download_onnx_files(
    repo: &HuggingfaceRepo,
    file_names: &[&str],
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let repo_interface = RepositoryInterface::new()?;
    let repository_info = repo_interface.get_repository_metadata(repo).await?;

    let mut files = vec![];
    let mut data_files = vec![];
    for file in file_names {
        files.push(file.to_string());
        let data_file: String = format!("{file}.data");
        if repository_info.files.contains(&data_file) {
            data_files.push(data_file);
        }
        let tensor_rt_file = file.replace(".onnx", ".trt.onnx");
        if repository_info.files.contains(&tensor_rt_file) {
            data_files.push(tensor_rt_file);
        }
    }
    let files: Vec<&str> = files
        .iter()
        .chain(data_files.iter())
        .map(|f| f.as_str())
        .collect();
    let downloaded_files = repository_info.download(&files).await?;
    let onnx_files: Vec<std::path::PathBuf> = downloaded_files
        .into_iter()
        .take(file_names.len())
        .collect();
    if onnx_files.len() == file_names.len() {
        return Ok(onnx_files);
    }
    Err(anyhow::anyhow!(
        "Expected to download {} ONNX files from {repo} but only received {}",
        file_names.len(),
        onnx_files.len()
    ))
}
