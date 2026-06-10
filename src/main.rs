mod ast;
mod codegen;
mod error;
mod lexer;
mod parser;
mod pipe;
mod sema;

use std::env::*;
use std::fs::*;
use std::path::Path;
use std::process::*;
use std::time::{Duration, Instant};

use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;

type Rtflags = (Option<String>, Option<String>, Vec<String>, bool);

fn main() {
    let args: Vec<String> = args().collect();

    Target::initialize_native(&InitializationConfig::default())
        .expect("failed to initialize native target");

    if args.len() < 2 {
        feed_error("No command provided. Try: 'astri help'", true);
    }

    let cmd = &args[1];

    match cmd.as_str() {
        "run" => run(&args),
        "build" => build(&args),
        "check" => check(&args),
        "dev" => dev(&args),
        "version" => feed_success("Version", "0.3.0"),
        "help" => usage(),
        _ => {
            feed_error(format!("'{}' is not a valid command.", cmd).as_str(), true);
        }
    }
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
                } else {
                    feed_error("Missing argument for option '-o / --output'.", true);
                }
            }
            "-n" | "--name" => {
                i += 1;
                if i < args.len() {
                    name = Some(args[i].clone());
                } else {
                    feed_error("Missing argument for option '-n / --name'.", true);
                }
            }
            "-k" | "--keep" => {
                keep = true;
            }
            arg if arg.starts_with('-') => {
                feed_error(format!("Unrecognized option '{}'.", arg).as_str(), true);
            }
            arg => {
                files.push(arg.to_string());
            }
        }
        i += 1;
    }
    (output, name, files, keep)
}

fn build(args: &[String]) {
    let timer = Instant::now();
    let (binary_path, _) = to_binary(args);
    time_it("Build", timer.elapsed());
    feed_success("Compiled", format!("~/{}", binary_path).as_str());
}

fn run(args: &[String]) {
    let timer = Instant::now();
    let (binary_path, _) = to_binary(args);
    time_it("Run", timer.elapsed());
    feed_success("Compiled", format!("~/{}", binary_path).as_str());
    let run_status = Command::new(&binary_path).status().unwrap_or_else(|e| {
        feed_error(
            format!("Failed to execute binary '{}': {}", binary_path, e).as_str(),
            true,
        );
        unreachable!()
    });
    exit(run_status.code().unwrap_or(1));
}

fn dev(args: &[String]) {
    let timer = Instant::now();
    let (_, name, files, keep) = flags(args);
    let file = match files.first() {
        Some(s) => s.as_str(),
        None => {
            feed_error("No input file specified.", true);
            unreachable!()
        }
    };
    let mod_name = name.as_deref().unwrap_or("main");
    let input = read_file(file);
    let stmts = compile(&input);

    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context, mod_name);
    if let Err(e) = codegen.compile(&stmts) {
        feed_error(format!("LLVM Code Generation failed: {}", e).as_str(), true);
    }

    let ll_path = file.replace(".ast", ".ll");
    codegen.module.print_to_file(&ll_path).unwrap();
    time_it("Compiler", timer.elapsed());
    feed_success("Iterated", format!("~/{}", ll_path).as_str());

    let status = Command::new("lli")
        .arg(&ll_path)
        .status()
        .expect("Failed to execute target detection command ('cc -dumpmachine'). Ensure Clang/GCC is installed.");

    if !keep {
        let _ = remove_file(&ll_path);
    }

    exit(status.code().unwrap_or(1));
}

fn check(args: &[String]) {
    let timer = Instant::now();
    let file = match args.get(2) {
        Some(s) => s.as_str(),
        None => {
            feed_error("No input file specified for 'check'.", true);
            unreachable!()
        }
    };
    let input =
        read_to_string(file).unwrap_or_else(|e| format!("Failed to read file '{}': {}", file, e));
    match pipe::run_pipe_that_runs_lex_then_parse_then_analyze_then_ok(&input) {
        Ok(_) => {
            time_it("Check", timer.elapsed());
            feed_success("_", "Everthing is OK");
        }
        Err((errors, warnings, info)) => {
            error::report_and_check(errors, warnings, info);
            exit(1);
        }
    }
}

fn read_file(path: &str) -> String {
    if !path.ends_with(".ast") {
        feed_error(
            format!(
                "File '{}' does not have the required '.ast' extension.",
                path
            )
            .as_str(),
            true,
        );
    }
    read_to_string(path).unwrap_or_else(|e| {
        feed_error(format!("Failed to open '{}': {}", path, e).as_str(), true);
        unreachable!()
    })
}

fn to_binary(args: &[String]) -> (String, bool) {
    let (output, name, files, keep) = flags(args);
    let file = match files.first() {
        Some(s) => s.as_str(),
        None => {
            feed_error("No input file specified.", true);
            unreachable!()
        }
    };
    let mod_name = name.as_deref().unwrap_or("main");
    let input = read_file(file);

    let stmts = compile(&input);
    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context, mod_name);
    if let Err(e) = codegen.compile(&stmts) {
        feed_error(
            format!("LLVM Code Generation failed: {:?}", e).as_str(),
            true,
        );
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
            feed_error(
                format!("Failed to create output directory hierarchy: {}", e).as_str(),
                true,
            );
        });
    }

    codegen.module.print_to_file(&ll_path).unwrap_or_else(|e| {
        feed_error(
            format!("Failed to write LLVM IR to '{}': {}", ll_path, e).as_str(),
            true,
        );
    });

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).expect("failed to get target");
    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .expect("failed to create target machine");

    target_machine
        .write_to_file(&codegen.module, FileType::Object, Path::new(&object_path))
        .expect("failed to write object file");

    link_object(&object_path, &binary_path);

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

fn link_object(o: &str, b: &str) {
    #[cfg(target_os = "windows")]
    {
        for linker in &["lld-link", "link.exe"] {
            let out = format!("/OUT:{}", b);
            if Command::new(linker)
                .args([&format!("/OUT:{}", b), o])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                return;
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        for linker in &["ld.lld", "lld", "cc"] {
            if Command::new(linker)
                .args([o, "-o", b])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                return;
            }
        }
    }

    feed_error("linking failed: no linker found", true);
}

// -----------------
use crate::error::Color;

fn time_it(id: &str, time: Duration) {
    let measure: String;
    if time.as_millis() < 100 {
        measure = format!("{}µs{}", Color::GREEN, Color::RESET)
    } else {
        measure = format!("{}ms{}", Color::GREEN, Color::RESET)
    };

    println!(
        "{}{:>14}{} finished in {} {}",
        Color::GREEN,
        id,
        Color::RESET,
        if time.as_millis() < 100 {
            time.as_micros()
        } else {
            time.as_millis()
        },
        measure.as_str(),
    );
}

fn feed_success(id: &str, msg: &str) {
    println!(
        "{}{:>14}{} {}{}{}",
        Color::GREEN,
        id,
        Color::RESET,
        Color::BOLD,
        msg,
        Color::RESET
    );
}

fn feed_error(msg: &str, is_exit: bool) {
    eprintln!(" {} Error {} {}", Color::ERROR, Color::RESET, msg);
    if is_exit {
        exit(1);
    }
}

fn usage() {
    eprintln!(
        "\n\t{} ✱{} asteri {}v0.3.0{}\n",
        "\x1b[1;45m",
        Color::RESET,
        Color::PURPLE,
        Color::RESET
    );
    eprintln!("  {}Usage:{}", Color::GREEN, Color::RESET);
    eprintln!("  {:>8} {}", "asteri", "<command> [options] <file>");
    eprintln!("");
    eprintln!("  {}Commands:{}", Color::GREEN, Color::RESET);
    eprintln!("  {:>8}\t{}", "run", "Compile and run the program");
    eprintln!("  {:>8}\t{}", "build", "Build the binary");
    eprintln!("  {:>8}\t{}", "check", "Lex, parse, and typecheck");
    eprintln!("  {:>8}\t{}", "dev", "For fast iteration");
    eprintln!("  {:>8}\t{}", "version", "Show version");
    eprintln!("  {:>8}\t{}", "help", "Display this message");
    eprintln!("");
    eprintln!("  {}Options:{}", Color::GREEN, Color::RESET);
    eprintln!(
        "\t{}\t{}",
        "-o --output <path>", "To specify an output path"
    );
    eprintln!(
        "\t{}\t{}",
        "-n --name <name>", "For naming the output files"
    );
    eprintln!("\t{}\t\t{}", "-k --keep", "Keep .ll and .o");
    eprintln!("");
}
