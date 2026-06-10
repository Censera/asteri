use std::process::Command;

#[test]
fn return_zero() {
    std::fs::create_dir_all("tests/bin").unwrap();

    let status = Command::new("cargo").args(["build"]).status().unwrap();
    assert!(status.success());

    let status = Command::new("./target/debug/asteri")
        .args([
            "build",
            "tests/return_zero.ast",
            "-n",
            "return_zero",
            "-o",
            "tests/bin",
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let binary = std::path::Path::new("tests/bin/return_zero");
    let output = Command::new(binary).output().unwrap();

    assert_eq!(output.status.code().unwrap(), 0);
}

#[test]
fn return_13() {
    std::fs::create_dir_all("tests/bin").unwrap();

    let status = Command::new("cargo").args(["build"]).status().unwrap();
    assert!(status.success());

    let status = Command::new("./target/debug/asteri")
        .args([
            "build",
            "tests/return_13.ast",
            "-n",
            "return_13",
            "-o",
            "tests/bin",
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let binary = std::path::Path::new("tests/bin/return_13");
    let output = Command::new(binary).output().unwrap();

    assert_eq!(output.status.code().unwrap(), 13);
}
