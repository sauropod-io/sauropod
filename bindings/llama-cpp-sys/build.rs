//! Build script for llama-cpp-sys crate

use std::env;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

const VERSION: &str = "5c0eb5ef544aeefd81c303e03208f768e158d93c";

/// Discover the CUDA installation path.
fn discover_cuda() -> Option<PathBuf> {
    println!("cargo:rerun-if-env-changed=CUDA_ROOT");
    if let Ok(cuda_home) = env::var("CUDA_HOME") {
        return Some(PathBuf::from(cuda_home));
    }

    let candidate_paths = &[PathBuf::from("/usr/local/cuda"), PathBuf::from("/opt/cuda")];
    for path in candidate_paths {
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }

    None
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let llama_source_dir = download_llama_source();
    let llama_build_dir = build_llama(&llama_source_dir);

    let lib_path = llama_build_dir.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    for library in ["ggml-base", "ggml-cpu", "ggml", "llama", "mtmd"] {
        println!("cargo:rustc-link-lib=static={library}");
    }

    // Link standard C++ library
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Generate bindings
    generate_bindings(&out_dir);
}

fn download_llama_source() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let extract_dir = out_dir.join(format!("llama.cpp-{VERSION}"));
    if extract_dir.exists() {
        return extract_dir;
    }

    let archive_path = out_dir.join("llama.tar.gz");
    let url = format!("https://github.com/ggml-org/llama.cpp/archive/{VERSION}.tar.gz");

    let agent = ureq::Agent::config_builder()
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .root_certs(ureq::tls::RootCerts::PlatformVerifier)
                .build(),
        )
        .build()
        .new_agent();

    let response = agent
        .get(&url)
        .call()
        .expect("Failed to download llama.cpp archive");

    let mut file =
        std::fs::File::create(&archive_path).expect("Failed to create llama.cpp archive file");
    std::io::copy(&mut response.into_body().into_reader(), &mut file)
        .expect("Failed to write llama.cpp archive");

    let file = std::fs::File::open(&archive_path).expect("Failed to open llama.cpp archive");
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive
        .unpack(&out_dir)
        .expect("Failed to extract llama.cpp archive");
    extract_dir
}

fn build_llama(source_dir: &Path) -> PathBuf {
    let mut cmake_config = cmake::Config::new(source_dir);

    // Make sure Clippy doesn't try to check llguidance
    if std::env::var_os("CLIPPY_ARGS").is_some() {
        unsafe {
            std::env::set_var(
                "CLIPPY_ARGS",
                "--allow=clippy::all__CLIPPY_HACKERY__--no-deps__CLIPPY_HACKERY__",
            );
        }
    }

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let rust_flags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let rustc_linker = std::env::var("RUSTC_LINKER").unwrap_or_default();
    let linker_plugin_lto = if rust_flags.contains("-Clinker-plugin-lto") {
        "ON"
    } else {
        "OFF"
    };

    // Configure CMake build
    cmake_config
        .cxxflag("-std=c++17")
        .define("CMAKE_LINKER", rustc_linker)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("GGML_OPENMP", "OFF")
        .define("LLAMA_BUILD_EXAMPLES", "OFF")
        .define("LLAMA_BUILD_SERVER", "OFF")
        .define("LLAMA_BUILD_TESTS", "OFF")
        .define("LLAMA_CURL", "OFF")
        .define("LLAMA_LLGUIDANCE", "OFF")
        .define("LLAMA_STATIC", "ON")
        .define("GGML_NATIVE", "OFF")
        .define("GGML_LTO", linker_plugin_lto)
        .define(
            "CMAKE_CUDA_ARCHITECTURES",
            if cfg!(feature = "cuda-multiple-arches") {
                "86;89;120"
            } else {
                "native"
            },
        );

    let target_cpu_regex =
        regex::Regex::new(r#"-Ctarget-cpu=((?:cortex|apple|neoverse)-[a-z0-9_]+|[a-z0-9_]+(?:-[0-9]+)?(?:-avx\d*|-v\d)?)"#)
            .expect("Failed to compile regex");
    if let Some(target_cpu) = target_cpu_regex
        .captures_iter(&rust_flags)
        .last()
        .and_then(|x| x.get(1))
        .map(|x| x.as_str())
    {
        if target_arch == "aarch64" {
            cmake_config.cflag(format!("-mcpu={target_cpu}"));
            cmake_config.cxxflag(format!("-mcpu={target_cpu}"));
        } else {
            cmake_config.cflag(format!("-march={target_cpu}"));
            cmake_config.cxxflag(format!("-march={target_cpu}"));
        }
    }

    if target_os == "macos" {
        cmake_config.define("GGML_METAL", "ON");
        cmake_config.define("GGML_METAL_USE_BF16", "ON");
        println!("cargo:rustc-link-lib=static=ggml-metal");
        println!("cargo:rustc-link-lib=static=ggml-blas");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    } else {
        if cfg!(feature = "cuda") {
            if let Some(cuda_path) = discover_cuda() {
                let target_dir = cuda_path
                    .join("targets")
                    .join(format!("{target_arch}-linux-gnu"));
                if target_dir.exists() {
                    println!(
                        "cargo:rustc-link-search=native={}/stubs",
                        target_dir.display()
                    );
                    println!("cargo:rustc-link-search=native={}", target_dir.display());
                }

                for lib_dir in &[cuda_path.join("lib64"), cuda_path.join("lib")] {
                    if lib_dir.exists() {
                        println!("cargo:rustc-link-search=native={}/stubs", lib_dir.display());
                        println!("cargo:rustc-link-search=native={}", lib_dir.display());
                    }
                }
            }

            cmake_config.define("GGML_CUDA", "ON");
            cmake_config.define("GGML_CUDA_F16", "ON");

            println!("cargo:rustc-link-lib=static=ggml-cuda");
            println!("cargo:rustc-link-lib=cuda");
            println!("cargo:rustc-link-lib=cudart");
            println!("cargo:rustc-link-lib=cublas");
        }

        if cfg!(feature = "vulkan") {
            println!("cargo:rustc-link-lib=static=ggml-vulkan");
            cmake_config.define("GGML_VULKAN", "ON");
            if target_os == "linux" {
                println!("cargo:rustc-link-lib=vulkan");
            } else if target_os == "windows" {
                println!("cargo:rustc-link-lib=vulkan-1");
            }
        }
    }

    let target_features: std::collections::HashSet<String> = std::collections::HashSet::from_iter(
        std::env::var("CARGO_CFG_TARGET_FEATURE")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .map(String::from),
    );
    for feature in &["sse4.2", "avx", "avx2", "avx512"] {
        if target_features.contains(*feature) {
            cmake_config.define(
                format!("GGML_{}", feature.replace('.', "").to_uppercase()),
                "ON",
            );
        }
    }

    // Build the project
    cmake_config.build()
}

fn generate_bindings(out_dir: &Path) {
    println!("cargo:rerun-if-changed=wrapper.hh");
    let mut builder = bindgen::Builder::default()
        .header("wrapper.hh")
        .clang_arg("-std=c++17")
        .allowlist_item("ggml_.*")
        .allowlist_item("gguf_.*")
        .allowlist_item("llama_.*")
        .allowlist_item("mtmd_.*")
        .rustified_enum(".*")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_copy(true)
        .derive_debug(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .generate_comments(true)
        .enable_function_attribute_detection()
        .generate_cstr(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // Include the installed include directory from the build
    let build_include_dir = out_dir.join("include");
    if build_include_dir.exists() {
        builder = builder.clang_arg(format!("-I{}", build_include_dir.display()));
    } else {
        println!(
            "cargo:warning=Include directory {} does not exist.",
            build_include_dir.display()
        );
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}
