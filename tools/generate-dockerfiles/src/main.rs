fn repo_root() -> std::path::PathBuf {
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

fn main() -> anyhow::Result<()> {
    let root = repo_root();
    let docker_directory = root.join("docker");
    for docker_file in docker_directory.read_dir()? {
        let docker_file = docker_file?;
        if docker_file
            .file_name()
            .to_string_lossy()
            .starts_with("Dockerfile.")
        {
            generate_dockerfiles::generate_code_for_docker(&docker_file.path())?;
        }
    }

    Ok(())
}
