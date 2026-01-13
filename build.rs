use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Injects "HELP" into src/main.rs
    prepare_documentation_code();
    // Generates README.md
    generate_doc("README.md", "readme.full");
    // Generates CHANGELOG.md
    generate_doc("CHANGELOG.md", "changelog.text");

    println!("cargo:rerun-if-changed=cue");
}

fn generate_doc(file_in_manifest: &str, eval_command: &str) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let readme_path = Path::new(&manifest_dir).join(file_in_manifest);
    let output = Command::new("cue")
        .args([
            "export",
            "-e",
            eval_command,
            "--out",
            "text",
            "./cue:documentation",
        ])
        .output()
        .expect("Failed to execute cue command");

    if !output.status.success() {
        panic!(
            "Cue {} generation failed:\n{}",
            file_in_manifest,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fs::write(&readme_path, &output.stdout).expect("Failed to write generated file");
}

fn prepare_documentation_code() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_docs.rs");

    let output = Command::new("cue")
        .args([
            "export",
            "-e",
            "help.code",
            "--out",
            "text",
            "./cue:documentation",
        ])
        .output()
        .expect("Failed to execute cue command");

    if !output.status.success() {
        panic!(
            "Cue generation failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fs::write(&dest_path, &output.stdout).expect("Failed to write generated file");
}
