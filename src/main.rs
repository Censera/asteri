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
use std::process::*;
use std::time::{Duration, Instant};

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
        "version" => println!("?"),
        _ => {
            eprintln!("error: '{}' is not a command", cmd);
            usage();
            exit(1);
        }
    }
}

fn usage() {
    eprintln!("\n\t\x1b[1;45m ✱\x1b[0m asteri\n");
}

fn flags(args: &[String]) -> (Option<String>, Option<String>, Vec<String>) {
    let mut output = None;
    let mut name = None;
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
            arg if !arg.starts_with('-') => break,
            _ => {}
        }
        i += 1;
    }
    let files: Vec<String> = args[i..].to_vec();

    (output, name, files)
}

fn build(args: &[String]) {
    let timer = Instant::now();
    let (output, name, files) = flags(args);
    let file = files.first().map(|s| s.as_str()).unwrap_or_else(|| {
        eprintln!("error: no input file");
        exit(1);
    });
    let mod_name = name.as_deref().unwrap_or("main");
    let input = read_file(file);

    let stmts = compile(&input);
    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context, mod_name);

    if let Err(e) = codegen.compile(&stmts) {
        eprintln!("code gen error: {}", e);
        exit(1);
    }

    let output_path = output.unwrap_or_else(|| file.replace(".ast", ".ll"));
    let output_path = if std::path::Path::new(&output_path).is_dir() {
        format!(
            "{}/{}",
            output_path,
            file.replace(".ast", ".ll").rsplit('/').next().unwrap()
        )
    } else {
        output_path
    };

    codegen.module.print_to_file(&output_path).unwrap();
    time_it("Builder", timer.elapsed());
}

fn run(args: &[String]) {
    let timer = Instant::now();
    let (output, name, files) = flags(args);
    let file = files.first().map(|s| s.as_str()).unwrap_or_else(|| {
        eprintln!("error: no input file");
        exit(1);
    });
    let mod_name = name.as_deref().unwrap_or("main");
    let input = read_file(file);

    let stmts = compile(&input);
    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context, mod_name);

    if let Err(e) = codegen.compile(&stmts) {
        eprintln!("code gen error: {}", e);
        exit(1);
    }

    let output_path = output.unwrap_or_else(|| file.replace(".ast", ".ll"));
    let output_path = if std::path::Path::new(&output_path).is_dir() {
        format!(
            "{}/{}",
            output_path,
            file.replace(".ast", ".ll").rsplit('/').next().unwrap()
        )
    } else {
        output_path
    };

    codegen.module.print_to_file(&output_path).unwrap();
    time_it("Builder", timer.elapsed());

    let status = Command::new("lli")
        .arg(&output_path)
        .status()
        .expect("failed to run lli");
    exit(status.code().unwrap_or(1));
}

fn check(args: &[String]) {
    let timer = Instant::now();
    let file = args.get(2).map(|s| s.as_str()).unwrap_or_else(|| {
        eprintln!("error: no input file");
        exit(1);
    });
    let input = read_to_string(file).unwrap_or_else(|e| {
        eprintln!("error reading '{}': {}", file, e);
        exit(1);
    });
    match pipe::run_pipe_that_runs_lex_then_parse_then_analyze_then_ok(&input) {
        Ok(_) => time_it("Checker", timer.elapsed()),
        Err((errors, warnings, info)) => {
            error::report_and_check(errors, warnings, info);
            exit(1);
        }
    }
}

fn read_file(path: &str) -> String {
    read_to_string(path).unwrap_or_else(|e| {
        eprintln!("error reading '{}': {}", path, e);
        exit(1);
    })
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
