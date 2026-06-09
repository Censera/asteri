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
        eprintln!("issue");
        exit(1);
    }

    let cmd = &args[1];

    match cmd.as_str() {
        "run" => run(&args, true),
        "build" => run(&args, false),
        "check" => check(&args),
        "version" => println!("?"),
        c => {
            eprintln!("error: '{}' is not a command", c);
            exit(1);
        }
    }
}

fn get_file(args: &[String]) -> &str {
    args.get(2).unwrap_or_else(|| {
        eprintln!("error: no input files");
        exit(1);
    })
}

fn run(args: &[String], exec: bool) {
    let timer = Instant::now();
    let file = get_file(args);
    let input = read_to_string(file).unwrap_or_else(|e| {
        eprintln!("error reading '{}': {}", file, e);
        exit(1);
    });

    let stmts = match pipe::run_pipe_that_runs_lex_then_parse_then_analyze_then_ok(&input) {
        Ok(s) => s,
        Err((errors, warnings, info)) => {
            error::report_and_check(errors, warnings, info);
            exit(1)
        }
    };

    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context, "main");
    if let Err(e) = codegen.compile(&stmts) {
        eprintln!("code gen error: {}", e);
        exit(1);
    }

    let output = file.replace(".ast", ".ll");
    codegen.module.print_to_file(&output).unwrap();

    if exec {
        let status = Command::new("lli")
            .args([output])
            .status()
            .expect("failed to run lli");
        time_it("Runner", timer.elapsed());
        exit(status.code().unwrap_or(1));
    } else {
        time_it("Builder", timer.elapsed());
    }
}

fn check(args: &[String]) {
    let timer = Instant::now();
    let file = get_file(args);
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

fn checking_result() {}

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
