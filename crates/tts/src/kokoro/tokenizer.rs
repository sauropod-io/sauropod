use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::Context;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct TokenizerJson {
    model: Model,
}

#[derive(Deserialize)]
struct Model {
    vocab: HashMap<String, u32>,
}

/// Tokenizer for the Kokoro TTS model.
///
/// This tokenizer mirrors the behaviour of the JavaScript implementation found
/// in the Kokoro repository. It first phonemizes the input text using
/// `espeak-ng` and then converts the resulting characters to token ids using the
/// vocabulary stored in `tokenizer.json`.
pub struct Tokenizer {
    vocab: HashMap<char, u32>,
    split_pattern: Regex,
}

impl Tokenizer {
    /// Load a tokenizer from the provided `tokenizer.json` file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path).context("reading tokenizer")?;
        let parsed: TokenizerJson = serde_json::from_str(&data)?;

        let vocab = parsed
            .model
            .vocab
            .into_iter()
            .filter_map(|(k, v)| k.chars().next().map(|c| (c, v)))
            .collect();

        // Same punctuation set as in phonemize.js
        const PUNCTUATION: &str = ";:,.!?¡¿—…\"«»“”(){}[]";
        let split_pattern = Regex::new(&format!(r"(\s*[{}]+\s*)+", regex::escape(PUNCTUATION)))?;

        Ok(Self {
            vocab,
            split_pattern,
        })
    }

    /// Convert a text segment to IPA using `espeak-ng`.
    fn espeak_segment(&self, text: &str) -> anyhow::Result<String> {
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        // TODO use a phonemization library instead of espeak-ng
        let mut child = Command::new("espeak-ng")
            .args(["-q", "--ipa=3", "-v", "en-us"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("spawning espeak-ng")?;

        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write as _;
            stdin.write_all(text.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        if !output.status.success() {
            anyhow::bail!("espeak-ng exited with {}", output.status);
        }
        let s = String::from_utf8(output.stdout)?;
        Ok(s.replace('\n', " ").trim().to_string())
    }

    /// Phonemize text by splitting on punctuation and running `espeak-ng` on the
    /// individual pieces.
    fn phonemize(&self, text: &str) -> anyhow::Result<String> {
        let mut result = String::new();
        let mut last = 0;
        for m in self.split_pattern.find_iter(text) {
            if last < m.start() {
                result.push_str(&self.espeak_segment(&text[last..m.start()])?);
            }
            result.push_str(m.as_str());
            last = m.end();
        }
        if last < text.len() {
            result.push_str(&self.espeak_segment(&text[last..])?);
        }
        Ok(result)
    }

    /// Tokenize the provided text.
    pub fn tokenize(&self, text: &str) -> anyhow::Result<Vec<u32>> {
        let phonemes = self.phonemize(text)?;
        let mut tokens = Vec::with_capacity(phonemes.len());
        for ch in phonemes.chars() {
            if let Some(&id) = self.vocab.get(&ch) {
                tokens.push(id);
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_tokenizer() -> anyhow::Result<()> {
        sauropod_tracing_test_helpers::init_tracing();

        let repo: sauropod_huggingface::HuggingfaceRepo =
            "huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX".parse()?;
        let repo_interface = sauropod_huggingface::RepositoryInterface::new()?;
        let repo_info = repo_interface.get_repository_metadata(&repo).await?;
        let files = repo_info.download(&["tokenizer.json"]).await?;
        let tokenizer = super::Tokenizer::from_file(&files[0])?;

        let text = "How could I know? It's an unanswerable question. Like asking an unborn child if they'll lead a good life. They haven't even been born.";

        let tokens = tokenizer.tokenize(text)?;
        let expected: Vec<u32> = vec![
            50, 157, 43, 135, 16, 53, 135, 46, 16, 43, 102, 16, 56, 156, 57, 135, 6, 16, 102, 62,
            61, 16, 70, 56, 16, 138, 56, 156, 72, 56, 61, 85, 123, 83, 44, 83, 54, 16, 53, 65, 156,
            86, 61, 62, 131, 83, 56, 4, 16, 54, 156, 43, 102, 53, 16, 156, 72, 61, 53, 102, 112,
            16, 70, 56, 16, 138, 56, 44, 156, 76, 158, 123, 56, 16, 62, 131, 156, 43, 102, 54, 46,
            16, 102, 48, 16, 81, 47, 102, 54, 16, 54, 156, 51, 158, 46, 16, 70, 16, 92, 156, 135,
            46, 16, 54, 156, 43, 102, 48, 4, 16, 81, 47, 102, 16, 50, 156, 72, 64, 83, 56, 62, 16,
            156, 51, 158, 64, 83, 56, 16, 44, 157, 102, 56, 16, 44, 156, 76, 158, 123, 56, 4,
        ];
        assert_eq!(tokens, expected);

        Ok(())
    }
}
