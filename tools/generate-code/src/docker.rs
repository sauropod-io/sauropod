use std::fmt::Write as _;

use anyhow::Context as _;

/// Use `git ls-files` to get a list of all files in the repository.
pub fn get_files() -> Vec<String> {
    let output = std::process::Command::new("git")
        .args(["ls-files"])
        .output()
        .expect("Failed to get files from git");

    if !output.status.success() {
        panic!(
            "Failed to get files from git: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout)
        .expect("Failed to convert output to string")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

const CARGO_MARKER: &str = "# CARGO MARKER";
const NPM_MARKER: &str = "# NPM MARKER";

/// Crawl the repository for `package.json` and `Cargo.toml` add `COPY`` lines for them in the `Dockerfile`.
pub fn generate_code_for_docker() -> anyhow::Result<()> {
    let root = crate::paths::repo_root();
    let dockerfile_path = root.join("Dockerfile");
    let dockerfile = std::fs::read_to_string(&dockerfile_path)
        .with_context(|| format!("Failed to read Dockerfile at {}", dockerfile_path.display()))?;
    let mut new_dockerfile_content = String::with_capacity(dockerfile.len());

    let files = get_files();
    let npm_copy = files
        .iter()
        .filter(|f| f.ends_with("package.json") || f.ends_with("package-lock.json"))
        .map(|x| format!("COPY {x} {x}\n"))
        .collect::<Vec<_>>();
    let cargo_files = files
        .iter()
        .filter(|f| f.ends_with("Cargo.toml") || f.ends_with("Cargo.lock"))
        .collect::<Vec<_>>();

    // Make empty `src/lib.rs` files for each `Cargo.toml` file to make `cargo fetch` happy.
    let mut make_fake_library_files = Vec::with_capacity(cargo_files.len());
    for cargo_file in &cargo_files {
        let path = std::path::Path::new(cargo_file);
        if let Some(parent) = path.parent() {
            if parent != path && parent.components().count() > 1 {
                make_fake_library_files.push(format!(
                    "    mkdir {0}/src && touch {0}/src/lib.rs && ",
                    parent.to_string_lossy().to_string()
                ));
            }
        }
    }

    let mut lines = dockerfile.lines();
    while let Some(line) = lines.next() {
        new_dockerfile_content.push_str(line);
        new_dockerfile_content.push('\n');

        let mut skip_till_empty = false;
        if line == CARGO_MARKER {
            skip_till_empty = true;
            for file in &cargo_files {
                writeln!(&mut new_dockerfile_content, "COPY {file} {file}")?;
            }
            writeln!(
                &mut new_dockerfile_content,
                "RUN {}",
                make_fake_library_files
                    .join("\\\n")
                    .trim()
                    .trim_end_matches(['&', ' '])
            )?;
        } else if line == NPM_MARKER {
            skip_till_empty = true;
            for copy in &npm_copy {
                new_dockerfile_content.push_str(copy);
            }
        }

        if skip_till_empty {
            new_dockerfile_content.push('\n');
            for line in lines.by_ref() {
                if line.is_empty() {
                    break;
                }
            }
        }
    }

    std::fs::write(&dockerfile_path, new_dockerfile_content).with_context(|| {
        format!(
            "Failed to write Dockerfile at {}",
            dockerfile_path.display()
        )
    })?;
    Ok(())
}
