#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use napi::{Error, Status};
use tokio::process::Command;

#[napi]
pub async fn run_yang_lint_in_non_strict_mode(
    yang_lint_binary_location: String,
    yang_files: Vec<String>,
    network_path: String,
) -> Result<String, napi::Error> {
    let output = Command::new(yang_lint_binary_location)
        .args(yang_files)
        .arg(network_path)
        .arg("-f")
        .arg("json")
        .arg("--not-strict")
        .kill_on_drop(true)
        .output()
        .await
        .expect("error");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Error::new(Status::GenericFailure, stderr))
    }
}

#[napi]
pub async fn run_yang_lint_in_strict_mode(
    yang_lint_binary_location: String,
    yang_files: Vec<String>,
    network_path: String,
) -> Result<String, napi::Error> {
    let output = Command::new(yang_lint_binary_location)
        .args(yang_files)
        .arg(network_path)
        .arg(">")
        .arg("/dev/null")
        .kill_on_drop(true)
        .output()
        .await
        .expect("error");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Error::new(Status::GenericFailure, stderr))
    }
}
