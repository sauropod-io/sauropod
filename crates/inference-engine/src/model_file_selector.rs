use regex::Regex;

/// TensorRT engine file extension.
pub(crate) const ENGINE_SUFFIX: &str = ".engine";
/// GGUF file extension.
pub(crate) const GGUF_SUFFIX: &str = ".gguf";
/// ONNX file extension.
pub(crate) const ONNX_SUFFIX: &str = ".onnx";

const DEFAULT_PREFERRED_QUANTIZATION: &str = "Q4_K_M";

pub(crate) fn select_file<'a>(
    files: &'a [String],
    preferred_quantization: Option<&str>,
) -> Option<&'a str> {
    // First priority: TensorRT engine files
    for file in files {
        if file.ends_with(ENGINE_SUFFIX) {
            return Some(file);
        }
    }

    // Second priority: GGUF files, preferring Q4_K_M quantization
    let mut gguf_files: Vec<&str> = files
        .iter()
        .filter(|file| file.ends_with(GGUF_SUFFIX))
        .map(|s| s.as_str())
        .collect();

    if !gguf_files.is_empty() {
        // Check if any GGUF file is a chunk (contains pattern like "00001-of-00002")
        let chunk_pattern = Regex::new(r"\d+-of-\d+").unwrap();

        // Sort GGUF files to prioritize Q4_K_M quantization
        gguf_files.sort_by(|a, b| {
            let a_has_q4km =
                a.contains(preferred_quantization.unwrap_or(DEFAULT_PREFERRED_QUANTIZATION));
            let b_has_q4km =
                b.contains(preferred_quantization.unwrap_or(DEFAULT_PREFERRED_QUANTIZATION));

            match (a_has_q4km, b_has_q4km) {
                (true, false) => std::cmp::Ordering::Less, // a comes first
                (false, true) => std::cmp::Ordering::Greater, // b comes first
                _ => a.cmp(b),                             // fallback to alphabetical order
            }
        });

        let gguf_file = gguf_files[0];
        if chunk_pattern.is_match(gguf_file) {
            // If it's a chunk file, return the parent directory
            return Some(gguf_file.rsplit_once('/').map(|x| x.0).unwrap_or("."));
        }

        return Some(gguf_file);
    }

    // Third priority: ONNX files
    files.iter().find_map(|file| {
        if file.ends_with(ONNX_SUFFIX) {
            Some(file.as_str())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_select_engine_file_priority() {
        let files = vec![
            "model.gguf".to_string(),
            "model.engine".to_string(),
            "model.onnx".to_string(),
        ];

        assert_eq!(select_file(&files, None), Some("model.engine"));
    }

    #[test]
    fn test_select_gguf_with_q4km_preference() {
        let files = vec![
            ".gitattributes".to_string(),
            "BF16/Llama-4-Scout-17B-16E-Instruct-BF16-00001-of-00005.gguf".to_string(),
            "BF16/Llama-4-Scout-17B-16E-Instruct-BF16-00002-of-00005.gguf".to_string(),
            "BF16/Llama-4-Scout-17B-16E-Instruct-BF16-00003-of-00005.gguf".to_string(),
            "BF16/Llama-4-Scout-17B-16E-Instruct-BF16-00004-of-00005.gguf".to_string(),
            "BF16/Llama-4-Scout-17B-16E-Instruct-BF16-00005-of-00005.gguf".to_string(),
            "IQ4_NL/Llama-4-Scout-17B-16E-Instruct-IQ4_NL-00001-of-00002.gguf".to_string(),
            "IQ4_NL/Llama-4-Scout-17B-16E-Instruct-IQ4_NL-00002-of-00002.gguf".to_string(),
            "IQ4_XS/Llama-4-Scout-17B-16E-Instruct-IQ4_XS-00001-of-00002.gguf".to_string(),
            "IQ4_XS/Llama-4-Scout-17B-16E-Instruct-IQ4_XS-00002-of-00002.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-Q2_K.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-Q2_K_L.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-Q3_K_S.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-UD-IQ1_M.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-UD-IQ1_S.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-UD-IQ2_M.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-UD-IQ2_XXS.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-UD-IQ3_XXS.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-UD-Q2_K_XL.gguf".to_string(),
            "Llama-4-Scout-17B-16E-Instruct-UD-Q3_K_XL.gguf".to_string(),
            "Q3_K_M/Llama-4-Scout-17B-16E-Instruct-Q3_K_M-00001-of-00002.gguf".to_string(),
            "Q3_K_M/Llama-4-Scout-17B-16E-Instruct-Q3_K_M-00002-of-00002.gguf".to_string(),
            "Q4_1/Llama-4-Scout-17B-16E-Instruct-Q4_1-00001-of-00002.gguf".to_string(),
            "Q4_1/Llama-4-Scout-17B-16E-Instruct-Q4_1-00002-of-00002.gguf".to_string(),
            "Q4_K_M/Llama-4-Scout-17B-16E-Instruct-Q4_K_M-00001-of-00002.gguf".to_string(),
            "Q4_K_M/Llama-4-Scout-17B-16E-Instruct-Q4_K_M-00002-of-00002.gguf".to_string(),
            "Q4_K_S/Llama-4-Scout-17B-16E-Instruct-Q4_K_S-00001-of-00002.gguf".to_string(),
            "Q4_K_S/Llama-4-Scout-17B-16E-Instruct-Q4_K_S-00002-of-00002.gguf".to_string(),
            "Q5_K_M/Llama-4-Scout-17B-16E-Instruct-Q5_K_M-00001-of-00002.gguf".to_string(),
            "Q5_K_M/Llama-4-Scout-17B-16E-Instruct-Q5_K_M-00002-of-00002.gguf".to_string(),
            "Q5_K_S/Llama-4-Scout-17B-16E-Instruct-Q5_K_S-00001-of-00002.gguf".to_string(),
            "Q5_K_S/Llama-4-Scout-17B-16E-Instruct-Q5_K_S-00002-of-00002.gguf".to_string(),
            "Q6_K/Llama-4-Scout-17B-16E-Instruct-Q6_K-00001-of-00002.gguf".to_string(),
            "Q6_K/Llama-4-Scout-17B-16E-Instruct-Q6_K-00002-of-00002.gguf".to_string(),
            "Q8_0/Llama-4-Scout-17B-16E-Instruct-Q8_0-00001-of-00003.gguf".to_string(),
            "Q8_0/Llama-4-Scout-17B-16E-Instruct-Q8_0-00002-of-00003.gguf".to_string(),
            "Q8_0/Llama-4-Scout-17B-16E-Instruct-Q8_0-00003-of-00003.gguf".to_string(),
            "README.md".to_string(),
            "UD-Q4_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q4_K_XL-00001-of-00002.gguf".to_string(),
            "UD-Q4_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q4_K_XL-00002-of-00002.gguf".to_string(),
            "UD-Q5_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q5_K_XL-00001-of-00002.gguf".to_string(),
            "UD-Q5_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q5_K_XL-00002-of-00002.gguf".to_string(),
            "UD-Q6_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q6_K_XL-00001-of-00002.gguf".to_string(),
            "UD-Q6_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q6_K_XL-00002-of-00002.gguf".to_string(),
            "UD-Q8_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q8_K_XL-00001-of-00003.gguf".to_string(),
            "UD-Q8_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q8_K_XL-00002-of-00003.gguf".to_string(),
            "UD-Q8_K_XL/Llama-4-Scout-17B-16E-Instruct-UD-Q8_K_XL-00003-of-00003.gguf".to_string(),
            "config.json".to_string(),
            "imatrix_unsloth.dat".to_string(),
            "mmproj-BF16.gguf".to_string(),
            "mmproj-F16.gguf".to_string(),
            "mmproj-F32.gguf".to_string(),
        ];

        assert_eq!(select_file(&files, None), Some("Q4_K_M"));
    }

    #[test]
    fn test_select_gguf_chunk_returns_parent_directory() {
        let files = vec![
            "model-00001-of-00002.gguf".to_string(),
            "model-00002-of-00002.gguf".to_string(),
            "config.json".to_string(),
        ];

        assert_eq!(select_file(&files, None), Some("."));
    }

    #[test]
    fn test_select_gguf_chunk_with_q4km_still_returns_parent() {
        let files = vec![
            "model-Q4_K_M-00001-of-00003.gguf".to_string(),
            "model-Q4_K_M-00002-of-00003.gguf".to_string(),
            "model-Q4_K_M-00003-of-00003.gguf".to_string(),
        ];

        assert_eq!(select_file(&files, None), Some("."));
    }

    #[test]
    fn test_chunk_pattern_detection() {
        let files = vec!["some-model-12345-of-67890.gguf".to_string()];

        assert_eq!(select_file(&files, None), Some("."));
    }

    #[test]
    fn test_select_gguf_fallback() {
        let files = vec!["model-Q8_0.gguf".to_string(), "model.onnx".to_string()];

        assert_eq!(select_file(&files, None), Some("model-Q8_0.gguf"));
    }

    #[test]
    fn test_select_onnx_fallback() {
        let files = vec!["model.onnx".to_string(), "other.txt".to_string()];

        assert_eq!(select_file(&files, None), Some("model.onnx"));
    }

    #[test]
    fn test_no_supported_files() {
        let files = vec!["model.txt".to_string(), "config.json".to_string()];

        assert_eq!(select_file(&files, None), None);
    }

    #[test]
    fn test_empty_files() {
        let files: Vec<String> = vec![];
        assert_eq!(select_file(&files, None), None);
    }
}
