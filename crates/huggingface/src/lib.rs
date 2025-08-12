use std::path::PathBuf;

use anyhow::Context;
use sauropod_config::HuggingfacePath;
use tokio::sync::Mutex;

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
    files: Mutex<Option<Vec<String>>>,
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

    pub async fn get_all_files(&self) -> anyhow::Result<Vec<String>> {
        let mut files = self.files.lock().await;
        if files.is_none() {
            let info = self
                .repository
                .info()
                .await
                .with_context(|| "Getting repository metadata".to_string())?;
            *files = Some(info.siblings.into_iter().map(|f| f.rfilename).collect())
        }

        Ok(files.as_ref().unwrap().clone())
    }
}

impl RepositoryInfo {
    /// Whether the repository has GGUF files.
    pub async fn has_gguf_files(&self) -> anyhow::Result<bool> {
        Ok(self
            .get_all_files()
            .await?
            .iter()
            .any(|f| f.ends_with(".gguf")))
    }

    /// Whether the repository has ONNX files.
    pub async fn has_onnx_files(&self) -> anyhow::Result<bool> {
        Ok(self
            .get_all_files()
            .await?
            .iter()
            .any(|f| f.ends_with(".onnx")))
    }

    /// Whether the repository has Safetensor files.
    pub async fn has_safetensor_files(&self) -> anyhow::Result<bool> {
        Ok(self
            .get_all_files()
            .await?
            .iter()
            .any(|f| f.ends_with(".safetensors")))
    }

    /// Whether the repository has TensorRT files.
    pub async fn has_tensorrt_files(&self) -> anyhow::Result<bool> {
        Ok(self
            .get_all_files()
            .await?
            .iter()
            .any(|f| f.ends_with(".engine")))
    }

    pub async fn get_model_storage_type(&self) -> anyhow::Result<Option<ModelStorageType>> {
        if self.has_onnx_files().await? {
            Ok(Some(ModelStorageType::ONNX))
        } else if self.has_gguf_files().await? {
            Ok(Some(ModelStorageType::GGUF))
        } else if self.has_tensorrt_files().await? {
            Ok(Some(ModelStorageType::TensorRT))
        } else if self.has_safetensor_files().await? {
            Ok(Some(ModelStorageType::Safetensors))
        } else {
            Ok(None)
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
        let hf_repo = make_hf_repo(repository.repo.clone(), repository.revision.clone());
        let model = self.api_client.repo(hf_repo.clone());
        let model_cache = self.cache.repo(hf_repo);

        Ok(RepositoryInfo {
            repository: model,
            model_cache,
            files: Mutex::new(None),
        })
    }
}

/// Download an ONNX file from a Hugging Face repository.
pub async fn download_onnx_files(
    repo: &HuggingfacePath,
    file_names: &[&str],
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let repo_interface = RepositoryInterface::new()?;
    let repository = make_hf_repo(repo.repo.clone(), repo.revision.clone());
    let model_cache = repo_interface.cache.repo(repository);

    // Check if all requested files are already in the cache.
    let mut missing_files = Vec::new();
    for file in file_names {
        if model_cache.get(file).is_none() {
            missing_files.push((*file).to_string());
        }
    }

    if missing_files.is_empty() {
        let mut onnx_files = Vec::with_capacity(file_names.len());
        for file in file_names {
            if let Some(path) = model_cache.get(file) {
                onnx_files.push(path);
            }
        }
        return Ok(onnx_files);
    }

    // Fetch metadata and download missing files and any associated data or TensorRT files.
    let repository_info = repo_interface.get_repository_metadata(repo).await?;
    let all_repository_files = repository_info.get_all_files().await?;
    let mut files_to_download = missing_files;
    for file in file_names {
        let data_file = format!("{file}.data");
        if all_repository_files.contains(&data_file) && model_cache.get(&data_file).is_none() {
            files_to_download.push(data_file);
        }
        let tensor_rt_file = file.replace(".onnx", ".trt.onnx");
        if all_repository_files.contains(&tensor_rt_file)
            && model_cache.get(&tensor_rt_file).is_none()
        {
            files_to_download.push(tensor_rt_file);
        }
    }

    if !files_to_download.is_empty() {
        let file_refs: Vec<&str> = files_to_download.iter().map(|f| f.as_str()).collect();
        repository_info.download(&file_refs).await?;
    }

    let mut onnx_files = Vec::with_capacity(file_names.len());
    for file in file_names {
        if let Some(path) = model_cache.get(file) {
            onnx_files.push(path);
        } else {
            return Err(anyhow::anyhow!(
                "Expected to download {} ONNX files from {repo} but could not find {}",
                file_names.len(),
                file
            ));
        }
    }

    Ok(onnx_files)
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
