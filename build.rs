#![doc = " Build script para trae-cli"]
#![doc = " Copia automáticamente el binario a bin/ después de compilar en release"]
use std::env;
use std::fs;
use std::path::Path;
#[doc = "Function documentation added by AI refactor"]
fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-arg=/STACK:8388608");
    }
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile == "release" {
        let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "cargo:warning=Could not determine CARGO_MANIFEST_DIR: {}",
                    e
                );
                return;
            }
        };
        let target_dir = Path::new(&manifest_dir).join("target").join("release");
        let bin_dir = Path::new(&manifest_dir).join("bin");
        if !bin_dir.exists() {
            let _ = fs::create_dir_all(&bin_dir);
        }
        let source = target_dir.join("trae.exe");
        let dest = bin_dir.join("trae.exe");
        if source.exists() {
            let _ = fs::copy(&source, &dest);
            println!("cargo:warning=✅ trae.exe copiado a bin/");
        }
    }
    println!("cargo:rerun-if-changed=src/main.rs");
}
