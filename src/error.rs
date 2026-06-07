pub struct Color;

impl Color {
    pub const RED: &'static str = "\x1b[1;31m";
    pub const ERROR: &'static str = "\x1b[1;31;41m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const WARNING: &'static str = "\x1b[1;30;43m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const INFO: &'static str = "\x1b[1;37;44m";
    pub const PURPLE: &'static str = "\x1b[1;95m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const RESET: &'static str = "\x1b[0m";
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorKind {
    Lexer,
    Parser,
    Sema,
    Warning,
    Info,
}

#[derive(Debug)]
#[allow(dead_code)]
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
        report_section("ERROR", Color::ERROR, Color::RED, &errors);
    }
    if !warnings.is_empty() {
        report_section("WARNING", Color::WARNING, Color::YELLOW, &warnings);
    }
    if !info.is_empty() {
        report_section("INFO", Color::INFO, Color::BLUE, &info);
    }
    !errors.is_empty()
}

fn report_section(header: &str, c1: &str, c2: &str, items: &[AsteriError]) {
    if items.is_empty() {
        return;
    }
    eprintln!("\t{} {} {}", c1, header, Color::RESET);
    for i in items {
        let msg = format!(
            "{}{:>4} | {}{}{}{} {}-> {}{}",
            Color::PURPLE,
            i.line,
            Color::RESET,
            Color::BOLD,
            i.src_line,
            Color::RESET,
            c2,
            i.msg,
            Color::RESET
        );
        let line_width = i.line.to_string().len().max(4);
        let spaces_len = line_width + 3; // 3 -> space + | + space
        let spaces = " ".repeat(spaces_len);
        let underline = "^".repeat(i.src_line.chars().count());

        eprintln!("{msg}\n{}{}{}{}", spaces, c2, underline, Color::RESET);
    }
}
