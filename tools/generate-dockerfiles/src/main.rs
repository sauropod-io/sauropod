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
    generate_dockerfiles::generate_code_for_docker(&docker_directory.join("Dockerfile.cuda"))?;
    generate_dockerfiles::generate_code_for_docker(&docker_directory.join("Dockerfile.vulkan"))?;

    Ok(())
}
