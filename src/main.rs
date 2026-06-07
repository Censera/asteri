mod ast;
mod error;
mod lexer;
mod parser;
mod sema;

use std::env::*;
use std::fs::*;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let is_debug = false;
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("\t<Use>\nac [file.ast]");
        return Ok(());
    }

    let file_path = &args[1];
    let input = read_to_string(file_path)?;

    // Lexer
    let start_lxr_time = Instant::now();
    let mut lexer = lexer::Lexer::new(&input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        let is_eof = matches!(token.kind, lexer::TokenKind::EOF);
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    let (errors, warnings, info) = lexer.take_all();
    let lxr_dur = start_lxr_time.elapsed();
    if error::report_and_check(errors, warnings, info) {
        return Ok(());
    }

    // Parser
    let start_prs_time = Instant::now();
    let mut parser = parser::Parser::new(tokens, &input);
    let stmts = parser.parse();
    let (errors, warnings, info) = parser.take_all();
    let prs_dur = start_prs_time.elapsed();
    if error::report_and_check(errors, warnings, info) {
        return Ok(());
    }

    // Sematic Analysis
    let start_sa_time = Instant::now();
    let mut sema = sema::Sema::new(&input);
    sema.analyze(&stmts);
    let (errors, warnings, info) = sema.take_all();
    let sa_dur = start_sa_time.elapsed();
    if error::report_and_check(errors, warnings, info) {
        return Ok(());
    }

    let total_dur = start_time.elapsed();
    if is_debug {
        let other_time = total_dur
            .checked_sub(lxr_dur + prs_dur + sa_dur)
            .unwrap_or(Duration::ZERO);

        debug("Lexer", total_dur, lxr_dur);
        debug("Parser", total_dur, prs_dur);
        debug("Semantic Analyser", total_dur, sa_dur);
        debug("Other", total_dur, other_time);

        println!(
            "\nin {}\x1b[1;95m milliseconds\x1b[0m:\n{:>10} Lines\n{:>10} Words\n{:>10} Bytes",
            start_time.elapsed().as_millis(),
            input.lines().count(),
            input.split_whitespace().count(),
            input.len()
        );
    } else {
        println!(
            "{}Compiled{} in {}{}",
            "\x1b[1;92m",
            "\x1b[0m",
            if total_dur.as_millis() < 100 {
                total_dur.as_micros()
            } else {
                total_dur.as_millis()
            },
            if total_dur.as_millis() < 100 {
                "\x1b[1;92m microseconds\x1b[0m"
            } else {
                "\x1b[1;92m milliseconds\x1b[0m"
            },
        );
    }

    Ok(())
}

fn debug(id: &str, total_time: Duration, start_time: Duration) {
    let end_time = start_time;

    let percentage = (end_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0;

    println!(
        "{}{:>18}{} {:02.}% finished in {}{}",
        "\x1b[1;95m",
        id,
        "\x1b[0m",
        percentage as i64,
        if end_time.as_millis() < 100 {
            end_time.as_micros()
        } else {
            end_time.as_millis()
        },
        if end_time.as_millis() < 100 {
            "\x1b[1;95m µs\x1b[0m"
        } else {
            "\x1b[1;95m ms\x1b[0m"
        },
    );
}
