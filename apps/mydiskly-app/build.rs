use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=assets/app.rc");
    println!("cargo:rerun-if-changed=assets/icon.ico");

    if let Ok(out_dir) = env::var("OUT_DIR") {
        let out_obj = PathBuf::from(&out_dir).join("mydiskly_icon_res.o");
        let status = Command::new("windres")
            .args([
                "-i",
                "assets/app.rc",
                "-I",
                "assets",
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
