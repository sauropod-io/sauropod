use std::path::PathBuf;

/// Get the URL for the pre-built ONNX Runtime library.
fn get_onnxruntime_url() -> String {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "1.22.0".to_string());
    let target_os = match std::env::var("CARGO_CFG_TARGET_OS")
        .unwrap_or_default()
        .as_str()
    {
        "macos" => "osx".to_string(),
        "windows" => "win".to_string(),
        x => x.to_string(),
    };

    let suffix = match target_os.as_str() {
        "win" => "zip",
        _ => "tgz",
    };
    let platform = match std::env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_default()
        .as_str()
    {
        "x86_64" if target_os == "osx" => "x86_64".to_string(),
        "x86_64" => "x64".to_string(),
        "arm64" if target_os == "osx" || target_os == "win" => "arm64".to_string(),
        platform => platform.to_string(),
    };

    let version_with_gpu = if platform != "osx" && platform != "aarch64" {
        format!("gpu-{version}")
    } else {
        version.clone()
    };

    format!(
        "https://github.com/microsoft/onnxruntime/releases/download/v{version}/onnxruntime-{target_os}-{platform}-{version_with_gpu}.{suffix}",
    )
}

fn download_and_extract_onnxruntime() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let url = get_onnxruntime_url();
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let onnxruntime_dir = out_dir.join("onnxruntime");

    // Skip download if already extracted
    if onnxruntime_dir.exists() {
        return Ok(onnxruntime_dir);
    }

    let agent = ureq::Agent::config_builder()
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .root_certs(ureq::tls::RootCerts::PlatformVerifier)
                .build(),
        )
        .build()
        .new_agent();
    let response = match agent.get(&url).call() {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Failed to download ONNX Runtime from {url}: {e}");
            return Err(e.into());
        }
    };
    let filename = url.split('/').next_back().unwrap();
    let archive_path = out_dir.join(filename);

    let mut file = std::fs::File::create(&archive_path)?;
    std::io::copy(&mut response.into_body().into_reader(), &mut file)?;

    extract_archive(&archive_path, &out_dir)?;

    // Find the extracted directory (it might have a version suffix)
    for entry in std::fs::read_dir(&out_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("onnxruntime")
        {
            std::fs::rename(&path, &onnxruntime_dir)?;
            break;
        }
    }

    Ok(onnxruntime_dir)
}

fn extract_archive(
    archive_path: &PathBuf,
    extract_to: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(archive_path)?;

    if archive_path.extension().unwrap() == "zip" {
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(extract_to)?;
    } else {
        // Handle .tgz files
        let tar = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(extract_to)?;
    }

    Ok(())
}

fn main() {
    let onnxruntime_result = pkg_config::probe_library("libonnxruntime");

    let (include_paths, link_paths, libs) = match onnxruntime_result {
        Ok(onnxruntime) => (
            onnxruntime.include_paths,
            onnxruntime.link_paths,
            onnxruntime.libs,
        ),
        _ => {
            let onnxruntime_dir = download_and_extract_onnxruntime()
                .expect("Failed to download and extract ONNX Runtime");

            let include_dir = onnxruntime_dir.join("include");
            let lib_dir = onnxruntime_dir.join("lib");

            // Copy shared libraries to OUT_DIR so they can be found at runtime
            let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
            // Terrible hack to get the top level output directory
            let shared_library_dir = out_dir
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap();
            for entry in std::fs::read_dir(&lib_dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                if matches!(
                    path.extension().and_then(|e| e.to_str()),
                    Some("so") | Some("so.1") | Some("dylib") | Some("dll")
                ) || filename.ends_with(".so.1")
                {
                    std::fs::copy(&path, shared_library_dir.join(&filename)).unwrap();
                }
            }

            (
                vec![include_dir],
                vec![lib_dir],
                vec!["onnxruntime".to_string()],
            )
        }
    };

    println!("cargo:rerun-if-changed=wrapper.hh");
    println!("cargo:rerun-if-changed=wrapper.cc");

    let mut builder = bindgen::Builder::default()
        .allowlist_item("Ort.*")
        .rustified_enum(".*")
        .clang_arg("-xc++")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_copy(false)
        .derive_debug(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .generate_comments(false)
        .enable_function_attribute_detection()
        .generate_cstr(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("wrapper.hh");

    let mut cc_builder = cc::Build::new();
    cc_builder.cpp(true);
    cc_builder.file("wrapper.cc").flag("-Wno-unused-variable");

    for include in include_paths {
        builder = builder.clang_arg(format!("-I{}", include.display()));
        cc_builder.include(&include);
    }
    for search_dir in link_paths {
        println!("cargo:rustc-link-search=native={}", search_dir.display());
    }
    for library in libs {
        println!("cargo:rustc-link-lib={library}");
    }

    cc_builder.compile("onnxruntime_bindings");

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
