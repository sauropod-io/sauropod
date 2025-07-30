use std::process::Command;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ReleaseInfo {
    #[serde(rename = "tagName")]
    tag_name: String,
}

fn main() -> anyhow::Result<()> {
    let matches = clap::Command::new("update-latest-image")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::new("repo")
                .long("repo")
                .short('r')
                .value_name("REPO")
                .default_value(
                    env!("CARGO_PKG_REPOSITORY")
                        .strip_prefix("https://github.com/")
                        .unwrap_or(""),
                )
                .help("The GitHub repository in the format <owner>/<repo>")
                .required(true),
        )
        .get_matches();

    let repo_name = matches
        .get_one::<String>("repo")
        .expect("required argument");

    // Get latest release information
    let output = Command::new("gh")
        .args(["release", "view", "-R", repo_name, "--json=tagName"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to get latest release: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let release_info: ReleaseInfo = serde_json::from_slice(&output.stdout)?;
    let Some(tag_name) = &release_info.tag_name.strip_prefix("v") else {
        anyhow::bail!(
            "Invalid tag name format - {} does not start with v",
            &release_info.tag_name
        );
    };

    println!("Latest release is {tag_name}");

    for suffix in ["vulkan", "cuda"] {
        // Define image names
        let repository = format!("ghcr.io/{repo_name}");
        let tagged_image = format!("{repository}:{tag_name}-{suffix}");
        let latest_image = format!("{repository}:latest-{suffix}");

        // Update the `latest` tag
        let status = Command::new("docker")
            .args([
                "buildx",
                "imagetools",
                "create",
                "--tag",
                &latest_image,
                &tagged_image,
            ])
            .status()?;
        if !status.success() {
            anyhow::bail!("Failed to create tag image");
        }
    }
    Ok(())
}
