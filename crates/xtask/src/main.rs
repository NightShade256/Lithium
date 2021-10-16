use std::{env::args, fs::remove_dir_all, process::Command};

fn build() {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--package")
        .arg("lithium")
        .status()
        .expect("failed to build WASM file");

    Command::new("wasm-bindgen")
        .arg("target/wasm32-unknown-unknown/release/lithium.wasm")
        .arg("--out-dir")
        .arg("static/pkg")
        .arg("--no-typescript")
        .arg("--target")
        .arg("web")
        .status()
        .expect("failed to run wasm-bindgen on WASM file");
}

fn clean() {
    remove_dir_all("static/pkg").expect("failed to remove `pkg` dir");
}

fn serve() {
    Command::new("python")
        .current_dir("static")
        .arg("-m")
        .arg("http.server")
        .status()
        .expect("failed to create an HTTP server with python's http.server module");
}

fn main() {
    if let Some(command) = args().nth(1) {
        match command.as_str() {
            "build" => build(),
            "clean" => clean(),
            "serve" => serve(),
            _ => eprintln!("error: invalid command `{}` provided", command),
        }
    } else {
        eprintln!("error: one of `build`, `serve` and `clean` expected");
    }
}
