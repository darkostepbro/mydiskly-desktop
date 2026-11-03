use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../mydiskly-app/assets/app.rc");
    println!("cargo:rerun-if-changed=../mydiskly-app/assets/icon.ico");
    println!("cargo:rerun-if-changed=../../target/release/MyDiskly.exe");

    // Ensure target/release/MyDiskly.exe exists so cargo check or workspace builds
    // do not fail immediately on a clean checkout before building mydiskly-app.
    let target_dir = Path::new("../../target/release");
    let target_exe = target_dir.join("MyDiskly.exe");
    if !target_exe.exists() {
        let _ = fs::create_dir_all(target_dir);
        let _ = fs::write(&target_exe, b"MZ_STUB_PLEASE_BUILD_MYDISKLY_APP_FIRST");
        println!("cargo:warning=target/release/MyDiskly.exe was not found. Created a placeholder stub. Please compile mydiskly-app first via 'cargo build --release -p mydiskly-app'!");
    }

    if let Ok(out_dir) = env::var("OUT_DIR") {
        let out_obj = PathBuf::from(&out_dir).join("mydiskly_installer_res.o");
        let status = Command::new("windres")
            .args([
                "-i",
                "../mydiskly-app/assets/app.rc",
                "-I",
                "../mydiskly-app/assets",
                "-o",
                &out_obj.to_string_lossy(),
            ])
            .status();

        if let Ok(s) = status {
            if s.success() && out_obj.exists() {
                println!("cargo:rustc-link-arg={}", out_obj.display());
            }
        }
    }
}
