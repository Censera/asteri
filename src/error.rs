pub struct Color;

impl Color {
    pub const RED: &'static str = "\x1b[1;31m";
    pub const ERROR: &'static str = "\x1b[1;31;41m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const WARNING: &'static str = "\x1b[1;30;43m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const INFO: &'static str = "\x1b[1;37;44m";
    pub const PURPLE: &'static str = "\x1b[1;35m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const RESET: &'static str = "\x1b[0m";
}

#[derive(Debug)]
pub enum ErrorKind {
    Lexer,
    Parser,
    Sema,
    Warning,
    Info,
}

#[derive(Debug)]
pub struct AsteriError {
    pub kind: ErrorKind,
    pub line: usize,
    pub src_line: String,
    pub msg: String,
}

impl AsteriError {
    pub fn new(kind: ErrorKind, line: usize, src_line: String, msg: String) -> Self {
        Self {
            kind,
            line,
            src_line,
            msg,
        }
    }
}

pub fn report_and_check(
    errors: Vec<AsteriError>,
    warnings: Vec<AsteriError>,
    info: Vec<AsteriError>,
) -> bool {
    if !errors.is_empty() {
        eprintln!("\t{} ERROR {}", Color::ERROR, Color::RESET);
        for e in &errors {
            let underline = "*".repeat(e.src_line.len());
            eprintln!(
                "{}{:<3}|{} {}{}{} {}\n     {} {}{}",
                Color::RED,
                e.line,
                Color::RESET,
                Color::BOLD,
                e.src_line,
                Color::RESET,
                Color::RED,
                underline,
                e.msg,
                Color::RESET
            );
        }
    }
    if !warnings.is_empty() {
        eprintln!("\t{} WARNING {}", Color::WARNING, Color::RESET);
        for w in &warnings {
            let underline = "-".repeat(w.src_line.len());
            eprintln!(
                "{}{:<3}|{} {}{}{} {}\n     {} {}{}",
                Color::YELLOW,
                w.line,
                Color::RESET,
                Color::BOLD,
                w.src_line,
                Color::RESET,
                Color::YELLOW,
                underline,
                w.msg,
                Color::RESET
            );
        }
    }
    if !info.is_empty() {
        eprintln!("\t{} INFO {}", Color::INFO, Color::RESET);
        for i in &info {
            let underline = "".repeat(i.src_line.len());
            eprintln!(
                "{}{:<3}|{} {}{}{} {}\n     {} {}{}",
                Color::BLUE,
                i.line,
                Color::RESET,
                Color::BOLD,
                i.src_line,
                Color::RESET,
                Color::BLUE,
                underline,
                i.msg,
                Color::RESET
            );
        }
    }
    !errors.is_empty()
}
