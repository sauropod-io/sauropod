use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    #[arg(short, long, help = "The Docker image to extract from")]
    image: String,

    #[arg(
        short,
        long,
        help = "Output tarball path",
        default_value = "sauropod-binaries.tar.gz"
    )]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Extracting binaries from Docker image: {}", args.image);

    // Create temporary directory for extraction
    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

    let temp_path = temp_dir.path();
    let container_name = format!("extract-{}", uuid::Uuid::new_v4().simple());

    // Create the container from the image
    println!("Creating container from image...");
    let output = Command::new("docker")
        .args(&[
            "create",
            "--name",
            &container_name,
            &args.image,
            "sleep",
            "60",
        ])
        .output()
        .context("Failed to create Docker container")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Docker create failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Ensure we clean up the container
    let _guard = ContainerGuard::new(&container_name);

    // Extract the binary
    let bin_path = temp_path.join("bin");
    fs::create_dir_all(&bin_path).context("Failed to create bin directory")?;

    println!("Extracting /bin/sauropod-inference-server...");
    let output = Command::new("docker")
        .args(&[
            "cp",
            &format!("{}:/bin/sauropod-inference-server", container_name),
            &bin_path.to_string_lossy(),
        ])
        .output()
        .context("Failed to extract sauropod-inference-server")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to extract binary: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Extract the libraries
    let lib_path = temp_path.join("lib");
    fs::create_dir_all(&lib_path).context("Failed to create lib directory")?;

    println!("Extracting /usr/lib/libonnxruntime* libraries...");

    // First, start the container temporarily to list the onnxruntime libraries
    let output = Command::new("docker")
        .args(&["start", &container_name])
        .output()
        .context("Failed to start container")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to start container: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let output = Command::new("docker")
        .args(&[
            "exec",
            &container_name,
            "find",
            "/usr/lib",
            "-name",
            "libonnxruntime*",
            "-type",
            "f",
        ])
        .output()
        .context("Failed to list onnxruntime libraries")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to find onnxruntime libraries: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let lib_files = String::from_utf8_lossy(&output.stdout);

    if lib_files.trim().is_empty() {
        return Err(anyhow!("No libonnxruntime* files found in /lib"));
    }

    // Extract each library file
    for lib_file in lib_files.lines() {
        let lib_file = lib_file.trim();
        if !lib_file.is_empty() {
            println!("Extracting {}...", lib_file);
            let filename = Path::new(lib_file)
                .file_name()
                .ok_or_else(|| anyhow!("Invalid library path: {}", lib_file))?;

            let output = Command::new("docker")
                .args(&[
                    "cp",
                    &format!("{}:{}", container_name, lib_file),
                    &lib_path.join(filename).to_string_lossy(),
                ])
                .output()
                .context("Failed to extract library file")?;

            if !output.status.success() {
                return Err(anyhow!(
                    "Failed to extract {}: {}",
                    lib_file,
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
    }

    // Create the tarball
    println!("Creating tarball: {}", args.output.display());
    create_tarball(&temp_path, &args.output)?;

    println!("Successfully created {}", args.output.display());
    Ok(())
}

fn create_tarball(source_dir: &Path, output_path: &PathBuf) -> Result<()> {
    let tar_gz = fs::File::create(output_path).context("Failed to create output file")?;

    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    tar.append_dir_all("bin", &source_dir.join("bin"))
        .context("Failed to add bin directory to tarball")?;
    tar.append_dir_all("lib", &source_dir.join("lib"))
        .context("Failed to add lib directory to tarball")?;

    tar.finish().context("Failed to finalize tarball")?;
    Ok(())
}

struct ContainerGuard {
    container_name: String,
}

impl ContainerGuard {
    fn new(container_name: &str) -> Self {
        Self {
            container_name: container_name.to_string(),
        }
    }
}

impl Drop for ContainerGuard {
    fn drop(&mut self) {
        let _ = Command::new("docker")
            .args(&["rm", "-f", &self.container_name])
            .output();
    }
}
