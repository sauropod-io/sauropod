use tokio::io::{AsyncRead, AsyncReadExt};

pub const CHAT_TEMPLATE_KEY: &str = "tokenizer.chat_template";
pub const ARCHITECTURE_KEY: &str = "general.architecture";

const MAX_STRING_LENGTH: u64 = 1_000_000;

#[derive(thiserror::Error, Debug)]
pub enum GgufError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid magic number: expected GGUF, got {0:?}")]
    InvalidMagic([u8; 4]),
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u32),
    #[error("Invalid value type: {0}")]
    InvalidValueType(u32),
    #[error("String too long: {0}")]
    StringTooLong(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GgufValue {
    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    String(String),
    Array(Vec<GgufValue>),
    UInt64(u64),
    Int64(i64),
    Float64(f64),
}

/// A key-value pair from GGUF metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct GgufMetadataEntry {
    pub key: String,
    pub value: GgufValue,
}

/// Parser to read metadata entries from a GGUF file.
pub struct GgufMetadataParser<R> {
    reader: R,
    remaining_kv_count: u64,
}

impl<R: AsyncRead + Unpin> GgufMetadataParser<R> {
    /// Creates a new `GgufMetadataParser`.
    pub async fn new(mut reader: R) -> Result<Self, GgufError> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).await?;
        if &magic != b"GGUF" {
            return Err(GgufError::InvalidMagic(magic));
        }

        // The version of the GGUF spec in use
        let version = reader.read_u32_le().await?;
        if version != 3 {
            return Err(GgufError::UnsupportedVersion(version));
        }

        // Read tensor count (we skip this for metadata-only parsing)
        let _tensor_count = reader.read_u64_le().await?;

        // Read metadata key-value count
        let remaining_kv_count = reader.read_u64_le().await?;

        Ok(GgufMetadataParser {
            reader,
            remaining_kv_count,
        })
    }

    /// Get the next metadata key-value pair. Returns None when no more metadata is available.
    pub async fn get_next(&mut self) -> Result<Option<GgufMetadataEntry>, GgufError> {
        if self.remaining_kv_count == 0 {
            return Ok(None);
        }

        // Read the next key-value pair
        let key = self.read_string().await?;
        let value = self.read_value().await?;
        self.remaining_kv_count -= 1;

        Ok(Some(GgufMetadataEntry { key, value }))
    }

    async fn read_string(&mut self) -> Result<String, GgufError> {
        let len = self.reader.read_u64_le().await?;
        if len > MAX_STRING_LENGTH {
            return Err(GgufError::StringTooLong(len));
        }

        let mut buf = vec![0u8; len as usize];
        self.reader.read_exact(&mut buf).await?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }

    async fn read_value(&mut self) -> Result<GgufValue, GgufError> {
        let value_type = self.reader.read_u32_le().await?;

        match value_type {
            0 => Ok(GgufValue::UInt8(self.reader.read_u8().await?)),
            1 => Ok(GgufValue::Int8(self.reader.read_i8().await?)),
            2 => Ok(GgufValue::UInt16(self.reader.read_u16_le().await?)),
            3 => Ok(GgufValue::Int16(self.reader.read_i16_le().await?)),
            4 => Ok(GgufValue::UInt32(self.reader.read_u32_le().await?)),
            5 => Ok(GgufValue::Int32(self.reader.read_i32_le().await?)),
            6 => Ok(GgufValue::Float32(self.reader.read_f32_le().await?)),
            7 => Ok(GgufValue::Bool(self.reader.read_u8().await? != 0)),
            8 => Ok(GgufValue::String(self.read_string().await?)),
            9 => {
                let array_type = self.reader.read_u32_le().await?;
                let array_len = self.reader.read_u64_le().await?;
                let mut array = Vec::new();

                for _ in 0..array_len {
                    let value = self.read_value_of_type(array_type).await?;
                    array.push(value);
                }

                Ok(GgufValue::Array(array))
            }
            10 => Ok(GgufValue::UInt64(self.reader.read_u64_le().await?)),
            11 => Ok(GgufValue::Int64(self.reader.read_i64_le().await?)),
            12 => Ok(GgufValue::Float64(self.reader.read_f64_le().await?)),
            _ => Err(GgufError::InvalidValueType(value_type)),
        }
    }

    async fn read_value_of_type(&mut self, value_type: u32) -> Result<GgufValue, GgufError> {
        match value_type {
            0 => Ok(GgufValue::UInt8(self.reader.read_u8().await?)),
            1 => Ok(GgufValue::Int8(self.reader.read_i8().await?)),
            2 => Ok(GgufValue::UInt16(self.reader.read_u16_le().await?)),
            3 => Ok(GgufValue::Int16(self.reader.read_i16_le().await?)),
            4 => Ok(GgufValue::UInt32(self.reader.read_u32_le().await?)),
            5 => Ok(GgufValue::Int32(self.reader.read_i32_le().await?)),
            6 => Ok(GgufValue::Float32(self.reader.read_f32_le().await?)),
            7 => Ok(GgufValue::Bool(self.reader.read_u8().await? != 0)),
            8 => Ok(GgufValue::String(self.read_string().await?)),
            10 => Ok(GgufValue::UInt64(self.reader.read_u64_le().await?)),
            11 => Ok(GgufValue::Int64(self.reader.read_i64_le().await?)),
            12 => Ok(GgufValue::Float64(self.reader.read_f64_le().await?)),
            _ => Err(GgufError::InvalidValueType(value_type)),
        }
    }
}

impl GgufMetadataParser<tokio::io::BufReader<tokio::fs::File>> {
    /// Creates a new `GgufMetadataParser` from a file.
    pub async fn from_file(file_path: &std::path::Path) -> Result<Self, GgufError> {
        let file = tokio::io::BufReader::new(tokio::fs::File::open(file_path).await?);
        Self::new(file).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_smallm2_metadata() {
        let repo = sauropod_config::HuggingfacePath {
            repo: "unsloth/SmolLM2-135M-Instruct-GGUF".to_string(),
            revision: None,
            path_or_quantization: None,
        };

        let repo_interface = sauropod_huggingface::RepositoryInterface::new().unwrap();
        let repository_info = repo_interface.get_repository_metadata(&repo).await.unwrap();
        let model_files = repository_info
            .download(&["SmolLM2-135M-Instruct-Q8_0.gguf"])
            .await
            .unwrap();
        let model_file = &model_files[0];

        let mut parser = GgufMetadataParser::from_file(model_file).await.unwrap();

        let expected_template = "{% for message in messages %}{% if loop.first and messages[0]['role'] != 'system' %}{{ '<|im_start|>system\nYou are a helpful AI assistant named SmolLM, trained by Hugging Face<|im_end|>\n' }}{% endif %}{{'<|im_start|>' + message['role'] + '\n' + message['content'] + '<|im_end|>' + '\n'}}{% endfor %}{% if add_generation_prompt %}{{ '<|im_start|>assistant\n' }}{% endif %}";
        let mut found_template = false;
        let mut found_architecture = false;
        while let Some(entry) = parser.get_next().await.unwrap() {
            if entry.key == CHAT_TEMPLATE_KEY {
                if let GgufValue::String(template) = entry.value {
                    assert_eq!(template, expected_template);
                    found_template = true;
                } else {
                    panic!("{CHAT_TEMPLATE_KEY} is not a string");
                }
            } else if entry.key == ARCHITECTURE_KEY {
                if let GgufValue::String(architecture) = entry.value {
                    assert_eq!(architecture, "llama");
                    found_architecture = true;
                } else {
                    panic!("{ARCHITECTURE_KEY} is not a string");
                }
            }
        }

        assert!(found_template, "{CHAT_TEMPLATE_KEY} not found");
        assert!(found_architecture, "{ARCHITECTURE_KEY} not found");
    }
}
