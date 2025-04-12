use std::env;
use std::io::Write;
use std::path::Path;

/// Get the MIME type for a given path.
fn mime_type(path: &Path) -> anyhow::Result<&'static str> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("js") => Ok("application/javascript"),
        Some("css") => Ok("text/css"),
        Some("html") => Ok("text/html"),
        Some("svg") => Ok("image/svg+xml"),
        Some("png") => Ok("image/png"),
        Some("gif") => Ok("image/gif"),
        Some("ico") => Ok("image/x-icon"),
        Some("woff") => Ok("font/woff"),
        Some("woff2") => Ok("font/woff2"),
        Some("ttf") => Ok("font/ttf"),
        _ => {
            anyhow::bail!("Could not detect file type for path: {}", path.display());
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Get the output directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("generated_ui_routes.rs");
    let mut generated_ui_routes = std::fs::File::create(&dest_path)?;

    // Get the path to the UI dist directory
    let ui_dist_dir = Path::new(&crate_dir).join("../../packages/ui/dist");

    if !ui_dist_dir.exists() {
        // Generate an error if the UI dist directory doesn't exist
        anyhow::bail!(
            "UI dist directory not found at {:?}, did you `npm run build`?",
            ui_dist_dir
        );
    }

    // Start writing the array
    writeln!(
        generated_ui_routes,
        "pub const FILES: &[(&str, &str, &[u8])] = &["
    )?;

    // Process all files in the UI dist directory recursively
    process_directory(&ui_dist_dir, &ui_dist_dir, &mut generated_ui_routes)?;

    // Close the array
    writeln!(generated_ui_routes, "];")?;

    // Rerun the build if any files in the UI dist directory change
    println!("cargo:rerun-if-changed=../../packages/ui/dist");

    Ok(())
}

/// Process a directory and write the file paths to the output file.
fn process_directory(
    base_dir: &Path,
    current_dir: &Path,
    output: &mut std::fs::File,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Process subdirectories
            process_directory(base_dir, &path, output)?;
        } else {
            let relative_path = path.strip_prefix(base_dir)?.as_os_str().to_str().unwrap();

            writeln!(
                output,
                r#"    ("/{}", "{}", include_bytes!("{}")),"#,
                // Re-map index.html to /
                if relative_path == "index.html" {
                    ""
                } else {
                    relative_path
                },
                mime_type(&path)?,
                path.display()
            )?;
        }
    }

    Ok(())
}
