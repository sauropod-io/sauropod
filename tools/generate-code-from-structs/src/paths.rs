//! Constants and static values.

pub fn get_crate_path(crate_name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("crates").join(crate_name)
}

pub fn get_api_path(file_or_directory: &str) -> std::path::PathBuf {
    std::path::PathBuf::from("api").join(file_or_directory)
}
