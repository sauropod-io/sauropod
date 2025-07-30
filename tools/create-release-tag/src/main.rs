use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn update_cargo_toml_version(content: &str, new_version: &str) -> anyhow::Result<String> {
    // Parse the TOML content
    let mut toml_doc = content.parse::<toml_edit::DocumentMut>()?;

    // Update the version in the [workspace.package] section
    let Some(workspace) = toml_doc.get_mut("workspace") else {
        anyhow::bail!("No workspace section found in Cargo.toml")
    };
    let Some(workspace) = workspace.as_table_mut() else {
        anyhow::bail!("Workspace section is not a table")
    };
    let Some(package) = workspace.get_mut("package") else {
        anyhow::bail!("No package section found in workspace")
    };
    if let Some(version) = package.as_table_mut().and_then(|t| t.get_mut("version")) {
        *version = toml_edit::value(new_version.to_string());
    } else {
        anyhow::bail!("No version field found in package section");
    }

    Ok(toml_doc.to_string())
}

fn main() -> anyhow::Result<()> {
    let matches = clap::Command::new("create-release")
        .about("Create a new release for the project")
        .subcommand(clap::Command::new("patch").about("Create a patch release (x.y.Z)"))
        .subcommand(clap::Command::new("minor").about("Create a minor release (x.Y.0)"))
        .subcommand(clap::Command::new("major").about("Create a major release (X.0.0)"))
        .get_matches();

    let current_version = env!("CARGO_PKG_VERSION").split('.').collect::<Vec<_>>();
    if current_version.len() != 3 {
        anyhow::bail!("Version string is not in the format x.y.z");
    }

    let major = current_version[0];
    let minor = current_version[1];
    let patch = current_version[2];

    let new_version = if matches.subcommand_matches("patch").is_some() {
        format!("{}.{}.{}", major, minor, patch.parse::<u32>()? + 1)
    } else if matches.subcommand_matches("minor").is_some() {
        format!("{}.{}.0", major, minor.parse::<u32>()? + 1)
    } else if matches.subcommand_matches("major").is_some() {
        format!("{}.0.0", major.parse::<u32>()? + 1)
    } else {
        anyhow::bail!("Please specify a release type: patch, minor, or major");
    };

    // Find the repository root using git
    let repo_root = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if !repo_root.status.success() {
        anyhow::bail!("Failed to find git repository root");
    }

    // Check if the repo has uncommitted changes
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;
    if !status.status.success() {
        anyhow::bail!("Failed to check git status");
    }
    if !status.stdout.is_empty() {
        anyhow::bail!(
            "There are uncommitted changes in the repository. Please commit or stash them before creating a release."
        );
    }

    let repo_root = String::from_utf8(repo_root.stdout)?.trim().to_string();

    // Update the version in `Cargo.toml`
    let cargo_toml_path = PathBuf::from(&repo_root).join("Cargo.toml");
    let cargo_lock_path = PathBuf::from(&repo_root).join("Cargo.lock");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;
    let updated_content = update_cargo_toml_version(&cargo_toml_content, &new_version)?;
    fs::write(&cargo_toml_path, updated_content)?;

    println!(
        "Updated version in {} to {new_version}",
        cargo_toml_path.display()
    );

    // Update the version in Cargo.lock by running cargo check
    let check_status = Command::new("cargo")
        .args(["check"])
        .current_dir(&repo_root)
        .status()?;
    if !check_status.success() {
        anyhow::bail!("cargo check failed");
    }

    // Commit the changes
    let add_status = Command::new("git")
        .args([
            "add",
            cargo_toml_path.to_str().unwrap(),
            cargo_lock_path.to_str().unwrap(),
        ])
        .status()?;
    if !add_status.success() {
        anyhow::bail!("Failed to add files to git");
    }

    let commit_message = format!("Release {new_version}");
    let commit_status = Command::new("git")
        .args([
            "commit",
            "-m",
            &commit_message,
            cargo_toml_path.to_str().unwrap(),
            cargo_lock_path.to_str().unwrap(),
        ])
        .status()?;
    if !commit_status.success() {
        anyhow::bail!("Failed to commit changes");
    }
    let tag_status = Command::new("git")
        .args(["tag", format!("v{new_version}").as_str()])
        .status()?;
    if !tag_status.success() {
        anyhow::bail!("Failed to create git tag");
    }

    println!(
        "Created tag v{new_version} - it's now ready for `git push origin main v{new_version}`"
    );

    Ok(())
}
