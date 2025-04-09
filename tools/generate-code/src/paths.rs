//! Constants and static values.

pub fn repo_root() -> std::path::PathBuf {
    std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .expect("Failed to get repo root")
        .stdout
        .into_iter()
        .map(|b| b as char)
        .collect::<String>()
        .trim()
        .into()
}

pub fn get_crate_path(crate_name: &str) -> std::path::PathBuf {
    repo_root().join("crates").join(crate_name)
}

pub fn get_api_path(file_or_directory: &str) -> std::path::PathBuf {
    repo_root().join("api").join(file_or_directory)
}

pub fn get_docs_path(file_or_directory: &str) -> std::path::PathBuf {
    repo_root().join("docs").join(file_or_directory)
}
