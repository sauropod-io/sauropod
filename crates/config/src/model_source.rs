#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum ConfigModelSource {
    /// A model from a local path.
    LocalPath(String),
    /// A model from a Hugging Face repository.
    HuggingFace(HuggingfacePath),
}

/// Either a path or a quantization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum PathOrQuantization {
    Quantization {
        /// The quantization type, for example "Q8_0".
        quantization: String,
    },
    FilePath {
        /// The file path, for example "model.onnx".
        file: String,
    },
}

/// A Huggingface repository in the form huggingface.co/<repo>@<revision>:<quantization>.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct HuggingfacePath {
    /// The repository name, for example "meta-llama/Llama-4-Scout-17B-16E-Instruct".
    pub repo: String,
    /// The revision of the repository, for example "main" or a specific commit hash.
    #[serde(default)]
    pub revision: Option<String>,
    /// The quantization or file path.
    #[serde(default, flatten)]
    pub path_or_quantization: Option<PathOrQuantization>,
}

impl std::str::FromStr for HuggingfacePath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(suffix) = s
            .strip_prefix("huggingface.co/")
            .or(s.strip_prefix("hf.co/"))
        {
            // First check for quantization (after :)
            let (prefix, path_or_quantization) =
                if let Some((prefix, quantization)) = suffix.split_once(':') {
                    (
                        prefix,
                        Some(PathOrQuantization::Quantization {
                            quantization: quantization.to_string(),
                        }),
                    )
                } else if let Some((prefix, file)) = suffix.split_once('/') {
                    // Then check for file path (after /)
                    (
                        prefix,
                        Some(PathOrQuantization::FilePath {
                            file: file.to_string(),
                        }),
                    )
                } else {
                    (suffix, None)
                };

            let (repo, version) = prefix
                .split_once('@')
                .map(|(prefix, version)| (prefix, Some(version.to_string())))
                .unwrap_or((prefix, None));
            Ok(HuggingfacePath {
                repo: repo.to_string(),
                revision: version,
                path_or_quantization,
            })
        } else {
            anyhow::bail!(
                "Invalid Huggingface repository - expected huggingface.co/<repo>@[revision]:[quantization], got: {}",
                s
            );
        }
    }
}

impl std::fmt::Display for HuggingfacePath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "huggingface.co/{}", self.repo)?;
        if let Some(revision) = &self.revision {
            write!(f, "@{revision}")?;
        }

        match &self.path_or_quantization {
            Some(PathOrQuantization::Quantization { quantization }) => {
                write!(f, ":{quantization}")?;
            }
            Some(PathOrQuantization::FilePath { file }) => {
                write!(f, "/{file}")?;
            }
            None => {}
        }

        Ok(())
    }
}

impl ConfigModelSource {
    /// Create a Hugging Face model source.
    pub fn from_huggingface(repo: &str, path_or_quantization: Option<PathOrQuantization>) -> Self {
        ConfigModelSource::HuggingFace(HuggingfacePath {
            repo: repo.to_string(),
            revision: None, // Revision is not set by default
            path_or_quantization,
        })
    }
}

impl std::str::FromStr for ConfigModelSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(huggingface_repo) = HuggingfacePath::from_str(s) {
            Ok(ConfigModelSource::HuggingFace(huggingface_repo))
        } else {
            Ok(ConfigModelSource::LocalPath(s.to_string()))
        }
    }
}

impl std::fmt::Display for ConfigModelSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigModelSource::LocalPath(path) => write!(f, "{}", path),
            ConfigModelSource::HuggingFace(repo) => write!(f, "{repo}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_source_from_str() {
        let local_model: ConfigModelSource = "path/to/local/model".parse().unwrap();
        let hf_model_1: ConfigModelSource = "huggingface.co/repo".parse().unwrap();
        let hf_model_2: ConfigModelSource = "huggingface.co/repo@revision".parse().unwrap();
        let hf_model_3: ConfigModelSource =
            "huggingface.co/repo@revision:quantization".parse().unwrap();
        let hf_model_4: ConfigModelSource = "huggingface.co/repo:quantization".parse().unwrap();
        let hf_model_5: ConfigModelSource = "huggingface.co/repo/file.onnx".parse().unwrap();

        assert_eq!(
            local_model,
            ConfigModelSource::LocalPath("path/to/local/model".to_string())
        );

        assert_eq!(
            hf_model_1,
            ConfigModelSource::HuggingFace(HuggingfacePath {
                repo: "repo".to_string(),
                revision: None,
                path_or_quantization: None
            })
        );
        assert_eq!(
            hf_model_2,
            ConfigModelSource::HuggingFace(HuggingfacePath {
                repo: "repo".to_string(),
                revision: Some("revision".to_string()),
                path_or_quantization: None
            })
        );
        assert_eq!(
            hf_model_3,
            ConfigModelSource::HuggingFace(HuggingfacePath {
                repo: "repo".to_string(),
                revision: Some("revision".to_string()),
                path_or_quantization: Some(PathOrQuantization::Quantization {
                    quantization: "quantization".to_string()
                })
            })
        );
        assert_eq!(
            hf_model_4,
            ConfigModelSource::HuggingFace(HuggingfacePath {
                repo: "repo".to_string(),
                revision: None,
                path_or_quantization: Some(PathOrQuantization::Quantization {
                    quantization: "quantization".to_string()
                })
            })
        );
        assert_eq!(
            hf_model_5,
            ConfigModelSource::HuggingFace(HuggingfacePath {
                repo: "repo".to_string(),
                revision: None,
                path_or_quantization: Some(PathOrQuantization::FilePath {
                    file: "file.onnx".to_string()
                })
            })
        );
    }
}
