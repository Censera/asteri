use std::fs::*;
use std::process::Command;

#[test]
fn return_zero() {
    test("return_zero", 0);
}

#[test]
fn return_13() {
    test("return_13", 13);
}

#[test]
fn cast_int_to_int() {
    std::fs::write(
        "tests/cast_int.ast",
        "fn: i32 main() {\n    let x: i64 = 42;\n    rt x -> i32;\n}\n",
    )
    .unwrap();
    test("cast_int", 42);
}

#[test]
fn cast_float_to_int() {
    std::fs::write(
        "tests/cast_float.ast",
        "fn: i32 main() {\n    let x: f64 = 3.14;\n    rt x -> i32;\n}\n",
    )
    .unwrap();
    test("cast_float", 3);
}

#[test]
fn cast_int_to_float() {
    std::fs::write(
        "tests/cast_float2.ast",
        "fn: i32 main() {\n    let x: i32 = 42;\n    let y: f64 = x -> f64;\n    rt y -> i32;\n}\n",
    )
    .unwrap();
    test("cast_float2", 42);
}

#[test]
fn cast_int_extend() {
    std::fs::write(
        "tests/cast_ext.ast",
        "fn: i64 main() {\n    let x: i32 = 42;\n    rt x -> i64;\n}\n",
    )
    .unwrap();
    test("cast_ext", 42);
}

#[test]
fn cast_int_truncate() {
    std::fs::write(
        "tests/cast_trunc.ast",
        "fn: i32 main() {\n    let x: i64 = 256;\n    rt x -> i8 -> i32;\n}\n",
    )
    .unwrap();
    test("cast_trunc", 0);
}

fn test(name: &str, exit: i32) {
    create_dir_all("tests/bin").unwrap();

    let source = format!("tests/{}.ast", name);
    let binary = format!("tests/bin/{}", name);

    let status = Command::new("cargo").args(["build"]).status().unwrap();
    assert!(status.success());

    let status = Command::new("./target/debug/asteri")
        .args(["build", &source, "-n", name, "-o", "tests/bin"])
        .status()
        .unwrap();
    assert!(status.success());

    let output = Command::new(&binary).output().unwrap();
    assert_eq!(output.status.code().unwrap(), exit);
}
