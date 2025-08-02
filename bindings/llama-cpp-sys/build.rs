//! Build script for llama-cpp-sys crate

use std::env;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

const VERSION: &str = "aa79524c51fb014f8df17069d31d7c44b9ea6cb8";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let llama_source_dir = download_llama_source();

    // Build llama.cpp using CMake
    let llama_build_dir = build_llama(&llama_source_dir);

    // Tell cargo to link the library
    // Try different possible library locations
    let lib_path = llama_build_dir.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    for library in ["ggml-base", "ggml-cpu", "ggml", "llama", "mtmd"] {
        println!("cargo:rustc-link-lib=static={library}");
    }

    // Link standard C++ library
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    }

    // Generate bindings
    generate_bindings(&llama_source_dir, &out_dir);
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
    let target_features: std::collections::HashSet<String> = std::collections::HashSet::from_iter(
        std::env::var("CARGO_CFG_TARGET_FEATURE")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .map(String::from),
    );

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let rust_flags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let rustc_linker = std::env::var("RUSTC_LINKER").unwrap_or_default();
    let target_cpu_regex =
        regex::Regex::new("-Ctarget-cpu=([a-z0-9]+)").expect("Failed to compile regex");

    let linker_plugin_lto = if rust_flags.contains("-Clinker-plugin-lto") {
        "ON"
    } else {
        "OFF"
    };

    // Configure CMake build
    cmake_config
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
        .define("CMAKE_CUDA_ARCHITECTURES", "native");

    if let Some(target_cpu) = target_cpu_regex
        .captures_iter(&rust_flags)
        .last()
        .and_then(|x| x.get(1))
    {
        cmake_config.cflag(format!("-march={}", target_cpu.as_str()));
        cmake_config.cxxflag(format!("-march={}", target_cpu.as_str()));
    }

    if target_os == "macos" {
        cmake_config.define("GGML_METAL", "ON");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=MetalKit");
    } else {
        if cfg!(feature = "cuda") {
            if let Some(cuda_path) = env::var_os("CUDA_HOME") {
                println!("cargo:rustc-link-search=native={}/lib", cuda_path.display());
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
            // Vulkan linking is handled by the Vulkan SDK
            if target_os == "linux" {
                println!("cargo:rustc-link-lib=vulkan");
            } else if target_os == "windows" {
                println!("cargo:rustc-link-lib=vulkan-1");
            }
        }
    }

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

fn generate_bindings(source_dir: &Path, out_dir: &Path) {
    println!("cargo:rerun-if-changed=wrapper.hh");
    let mut builder = bindgen::Builder::default()
        .header("wrapper.hh")
        .allowlist_item("gguf_.*")
        .allowlist_item("ggml_.*")
        .allowlist_item("llama_.*")
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

    // Add include directories - both source and build directories
    builder = builder.clang_arg(format!("-I{}", source_dir.join("include").display()));
    builder = builder.clang_arg(format!(
        "-I{}",
        source_dir.join("ggml").join("include").display()
    ));
    builder = builder.clang_arg(format!("-I{}", source_dir.display()));

    // Also try the installed include directory from the build
    let build_include_dir = out_dir.join("include");
    if build_include_dir.exists() {
        builder = builder.clang_arg(format!("-I{}", build_include_dir.display()));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}
