use std::fs::*;
use std::process::Command;

#[test]
fn return_zero() {
    test_exit("return_zero", 0);
}

#[test]
fn return_13() {
    test_exit("return_13", 13);
}

#[test]
fn cast_int_to_int() {
    write(
        "tests/cast_int.ast",
        "fn: int main() {\n    let x: i8 = 7;\n    return x -> i32; // int <-> i32\n}\n",
    )
    .unwrap();
    test_exit("cast_int", 7);
}

#[test]
fn cast_float_to_int() {
    write(
        "tests/cast_float.ast",
        "fn: i32 main() {\n    let x: f64 = 3.14;\n    return x -> i32;\n}\n",
    )
    .unwrap();
    test_exit("cast_float", 3);
}

#[test]
fn cast_int_to_float() {
    write(
        "tests/cast_float2.ast",
        "fn: i32 main() {\n    let x: i32 = 42;\n    let y: f64 = x -> f64;\n    return y -> i32;\n}\n",
    )
    .unwrap();
    test_exit("cast_float2", 42);
}

#[test]
fn cast_int_extend() {
    write(
        "tests/cast_ext.ast",
        "fn: i64 main() {\n    let x: i32 = 42;\n    return x -> i64;\n}\n",
    )
    .unwrap();
    test_exit("cast_ext", 42);
}

#[test]
fn cast_int_truncate() {
    write(
        "tests/cast_trunc.ast",
        "fn: i32 main() {\n    let x: i64 = 256;\n    return x -> i8 -> i32;\n}\n",
    )
    .unwrap();
    test_exit("cast_trunc", 0);
}

#[test]
fn print_hello() {
    write(
        "tests/print_hello.ast",
        "fn main() {\n    print \"Hello, world!\";\n}\n",
    )
    .unwrap();
    check_strout("print_hello", "Hello, world!");
}

#[test]
fn print_number() {
    write("tests/print_number.ast", "fn main() {\n    print 13;\n}\n").unwrap();
    check_strout("print_number", "13");
}

#[test]
fn print_string() {
    write(
        "tests/print_string.ast",
        "fn main() {\n    let s: str = \"This is a str\";\n    print s;\n}\n",
    )
    .unwrap();
    check_strout("print_string", "This is a str");
}

fn test_exit(name: &str, exit: i32) {
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

fn check_strout(name: &str, out: &str) {
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
    assert_eq!(output.status.code().unwrap(), 0);
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), out);
}
