use std::{env, process::Command};

#[test]
fn simple() {
    if let Ok(path) = env::current_dir() {
        assert!(path.ends_with("claw_verify"));
    } else {
        println!("Could not verify current directory is claw_verify.");
    }

    let status = Command::new("cargo")
        .arg("run")
        .arg("tests/simple.json")
        .status()
        .expect("Failed to run command.");

    assert!(status.success());
}
