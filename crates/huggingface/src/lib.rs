use std::path::PathBuf;

use sauropod_config::HuggingfacePath;

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

/// Make an API client for Hugging Face.
fn make_api_client() -> Result<hf_hub::api::tokio::Api, hf_hub::api::tokio::ApiError> {
    let builder = hf_hub::api::tokio::ApiBuilder::from_env().high();
    if let Ok(token) = std::env::var("HF_TOKEN") {
        builder.with_token(Some(token)).build()
    } else {
        builder.build()
    }
}

fn make_hf_repo(repository: String, revision: Option<String>) -> hf_hub::Repo {
    match revision {
        Some(revision) => {
            hf_hub::Repo::with_revision(repository, hf_hub::RepoType::Model, revision)
        }
        None => hf_hub::Repo::model(repository),
    }
}

impl RepositoryInterface {
    pub fn new() -> anyhow::Result<Self> {
        let client = make_api_client()?;
        let cache = hf_hub::Cache::from_env();
        Ok(Self {
            api_client: client,
            cache,
        })
    }

    pub async fn get_repository_metadata(
        &self,
        repository: &HuggingfacePath,
    ) -> anyhow::Result<RepositoryInfo> {
        let repository = make_hf_repo(repository.repo.clone(), repository.revision.clone());
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
    repo: &HuggingfacePath,
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

/// Download a model from Hugging Face or return its path.
pub async fn download_file(
    model_source: &sauropod_config::ConfigModelSource,
) -> anyhow::Result<std::path::PathBuf> {
    match model_source {
        sauropod_config::ConfigModelSource::LocalPath(path) => Ok(std::path::PathBuf::from(path)),
        sauropod_config::ConfigModelSource::HuggingFace(sauropod_config::HuggingfacePath {
            repo,
            revision,
            path_or_quantization,
        }) => match path_or_quantization {
            Some(sauropod_config::PathOrQuantization::FilePath { file }) => {
                let api_client = make_api_client()?;
                let repository = make_hf_repo(repo.clone(), revision.clone());
                let api_repo = api_client.repo(repository.clone());
                Ok(api_repo.get(file.as_str()).await?)
            }
            Some(sauropod_config::PathOrQuantization::Quantization { .. }) => {
                anyhow::bail!(
                    "Determining file from quantization not supported for {model_source}"
                );
            }
            None => {
                anyhow::bail!("No file path specified for {model_source}");
            }
        },
    }
}
