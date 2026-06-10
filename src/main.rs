mod ast;
mod codegen;
mod error;
mod lexer;
mod parser;
mod pipe;
mod sema;

use inkwell::context::Context;
use std::env::*;
use std::fs::*;
use std::path::Path;
use std::process::*;
use std::time::{Duration, Instant};

type Rtflags = (Option<String>, Option<String>, Vec<String>, bool);

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        usage();
        exit(1);
    }

    let cmd = &args[1];

    match cmd.as_str() {
        "run" => run(&args),
        "build" => build(&args),
        "check" => check(&args),
        "version" => feed_success("version", "0.2.5"),
        _ => {
            usage();
            feed_error(format!("'{}' is not a command", cmd).as_str(), true);
        }
    }
}

fn usage() {
    eprintln!("\n\t\x1b[1;45m ✱\x1b[0m asteri\n");
}

fn flags(args: &[String]) -> Rtflags {
    let mut output = None;
    let mut name = None;
    let mut keep = false;
    let mut files = Vec::new();

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                i += 1;
                if i < args.len() {
                    output = Some(args[i].clone());
                }
            }
            "-n" | "--name" => {
                i += 1;
                if i < args.len() {
                    name = Some(args[i].clone());
                }
            }
            "-k" | "--keep" => {
                keep = true;
            }
            arg if !arg.starts_with('-') => {
                files.push(arg.to_string());
            }
            _ => {}
        }
        i += 1;
    }
    (output, name, files, keep)
}

fn build(args: &[String]) {
    let timer = Instant::now();
    let (binary_path, _) = to_binary(args);
    time_it("Builder", timer.elapsed());
    feed_success("Compiled", format!("{}", binary_path).as_str());
}

fn run(args: &[String]) {
    let timer = Instant::now();
    let (binary_path, _) = to_binary(args);
    time_it("Runner", timer.elapsed());
    feed_success("Compiled", format!("{}", binary_path).as_str());
    let run_status = Command::new(&binary_path)
        .status()
        .expect("failed to run binary");
    exit(run_status.code().unwrap_or(1));
}

fn check(args: &[String]) {
    let timer = Instant::now();
    let file = args
        .get(2)
        .map(|s| s.as_str())
        .unwrap_or_else(|| "no input file");
    let input = read_to_string(file).unwrap_or_else(|e| format!("'{}' {}", file, e));
    match pipe::run_pipe_that_runs_lex_then_parse_then_analyze_then_ok(&input) {
        Ok(_) => time_it("Checker", timer.elapsed()),
        Err((errors, warnings, info)) => {
            error::report_and_check(errors, warnings, info);
            exit(1);
        }
    }
}

fn read_file(path: &str) -> String {
    if !path.ends_with(".ast") {
        feed_error("file format not recognized", true);
    }
    read_to_string(path).unwrap_or_else(|e| {
        feed_error(format!("'{}' {}", path, e).as_str(), true);
        unreachable!()
    })
}

fn to_binary(args: &[String]) -> (String, bool) {
    let (output, name, files, keep) = flags(args);
    let file = files
        .first()
        .map(|s| s.as_str())
        .unwrap_or_else(|| "no input file");
    let mod_name = name.as_deref().unwrap_or("main");
    let input = read_file(file);

    let stmts = compile(&input);
    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context, mod_name);
    if let Err(e) = codegen.compile(&stmts) {
        feed_error(format!("Code gen failed '{}'", e).as_str(), true);
    }

    let base_dir = file.rsplit_once('/').map(|(d, _)| d).unwrap_or(".");
    let ll_path;
    let object_path;
    let binary_path;

    if let Some(out) = &output {
        let output_path = Path::new(out);
        if output_path.is_dir() {
            ll_path = format!("{}/{}.ll", out, mod_name);
            object_path = format!("{}/{}.o", out, mod_name);
            binary_path = format!("{}/{}", out, mod_name);
        } else {
            binary_path = out.clone();
            let parent = output_path.parent().unwrap_or(Path::new("."));
            ll_path = format!("{}/{}.ll", parent.display(), mod_name);
            object_path = format!("{}/{}.o", parent.display(), mod_name);
        }
    } else {
        ll_path = format!("{}/{}.ll", base_dir, mod_name);
        object_path = format!("{}/{}.o", base_dir, mod_name);
        binary_path = format!("{}/{}", base_dir, mod_name);
    }

    if let Some(parent) = Path::new(&ll_path).parent() {
        create_dir_all(parent).unwrap_or_else(|e| {
            feed_error(format!("creating output directory '{}'", e).as_str(), true);
        });
    }
    codegen.module.print_to_file(&ll_path).unwrap();

    let trip = String::from_utf8(
        Command::new("cc")
            .arg("-dumpmachine")
            .output()
            .expect("failed to detect target triple")
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string();

    let llc_status = Command::new("llc")
        .arg("-mtriple")
        .arg(&trip)
        .arg("-relocation-model=pic")
        .arg(&ll_path)
        .arg("-o")
        .arg(&object_path)
        .arg("-filetype=obj")
        .status()
        .expect("failed to run llc");
    if !llc_status.success() {
        feed_error("llc failed", true);
    }

    let cc_status = Command::new("cc")
        .arg(&object_path)
        .arg("-o")
        .arg(&binary_path)
        .status()
        .expect("failed to run cc");
    if !cc_status.success() {
        feed_error("cc linking failed", true);
    }

    if !keep {
        let _ = remove_file(&ll_path);
        let _ = remove_file(&object_path);
    }

    (binary_path, keep)
}

fn compile(input: &str) -> Vec<ast::Stmt> {
    match pipe::run_pipe_that_runs_lex_then_parse_then_analyze_then_ok(&input) {
        Ok(stmts) => stmts,
        Err((errors, warnings, info)) => {
            error::report_and_check(errors, warnings, info);
            exit(1)
        }
    }
}

//fn checking_result() {}

fn time_it(id: &str, time: Duration) {
    println!(
        "\x1b[1;92m{:>14}\x1b[0m finished in {}{}",
        id,
        if time.as_millis() < 100 {
            time.as_micros()
        } else {
            time.as_millis()
        },
        if time.as_millis() < 100 {
            "\x1b[1;92m µs\x1b[0m"
        } else {
            "\x1b[1;92m ms\x1b[0m"
        },
    );
}

fn feed_success(id: &str, msg: &str) {
    println!("\x1b[1;92m{:>14}\x1b[0m {}", id, msg);
}

fn feed_error(msg: &str, is_exit: bool) {
    eprintln!(" \x1b[1;41m ERROR \x1b[0m {}", msg);
    if is_exit {
        exit(1);
    }
}
