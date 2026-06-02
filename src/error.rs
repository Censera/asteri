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

    pub fn report(&self) {
        let (label, color1, color2) = match self.kind {
            ErrorKind::Lexer => ("ERROR", Color::ERROR, Color::RED),
            ErrorKind::Parser => ("ERROR", Color::ERROR, Color::RED),
            ErrorKind::Sema => ("ERROR", Color::ERROR, Color::RED),
            ErrorKind::Warning => ("WARNING", Color::WARNING, Color::YELLOW),
            ErrorKind::Info => ("INFO", Color::INFO, Color::BLUE),
        };

        eprintln!(
            "\t{} {} {}\n{} | <- {}{}",
            color1,
            label,
            Color::RESET,
            self.line,
            self.msg,
            Color::RESET
        );
    }
}

pub fn report_and_check(
    errors: Vec<AsteriError>,
    warnings: Vec<AsteriError>,
    info: Vec<AsteriError>,
) -> bool {
    for e in &errors {
        e.report();
    }
    for w in &warnings {
        w.report();
    }
    for i in &info {
        i.report();
    }
    !errors.is_empty()
}
