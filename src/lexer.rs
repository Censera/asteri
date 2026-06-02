use crate::error::{AsteriError, ErrorKind};
use std::fmt;

// Tokens
#[derive(Debug)]
pub enum TokenKind {
    // Literals
    Int(i64),
    Str(String),
    Char(char),
    Float(f64),

    // Types
    Type(Types),

    // Operators
    Asterisk,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Huh,
    Percent,
    At,
    Caret,
    Ampersand,
    Minus,
    Plus,
    Equal,
    Slash,
    Pipe,
    Tilde,
    Bang,
    LessThan,
    GreaterThan,
    OpeningRound,
    ClosingRound,
    OpeningCurly,
    ClosingCurly,
    OpeningSquare,
    ClosingSquare,

    // Compound Char
    Arrow,
    NotEqual,
    EqualEqual,
    LessOrEqual,
    GreaterOrEqual,
    TwoPipes,
    TwoAmpersands,
    ShiftLeft,
    ShiftRight,

    // Keywords
    Let,
    Immut,
    If,
    Else,
    While,
    Loop,
    Match,
    Break,
    Ret,
    Continue,
    True,
    False,
    Null,
    New,
    Form,
    Fun,
    Pub,
    Pri,
    Enum,
    Struct,
    Use,
    Print,
    Write,
    Read,
    Append,
    Error,
    Open,
    Close,
    Sort,

    BitXor,
    BitOr,
    BitAnd,
    BitNot,

    // Other
    EOF,
    Id(String),
    CBlock,
}

#[derive(Debug)]
pub struct TextSpan {
    start: usize,
    end: usize,
    pub literal: String,
    pub line: usize,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String, line: usize) -> Self {
        Self {
            start,
            end,
            literal,
            line,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    Bool,
    Char,
    File,
    Matrix3x3,
    Matrix4x4,
    Vector2,
    Vector3,

    Pointer(Box<Types>),
    OptionPointer(Box<Types>),
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer {
    chars: Vec<char>,
    current: usize,
    line: usize,
    errors: Vec<AsteriError>,
    warnings: Vec<AsteriError>,
    info: Vec<AsteriError>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            current: 0,
            line: 1,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while matches!(self.current_char(), Some(c) if c.is_whitespace()) {
            self.swallow();
        }

        if self.current_char() == Some('-')
            && self.chars.get(self.current + 1).copied() == Some('-')
        {
            while self.current_char() != Some('\n') && !self.is_eof() {
                self.swallow();
            }
            return self.next_token();
        }

        let c = match self.current_char() {
            Some(c) => c,
            None => return None,
        };

        let token_line = self.line;
        let start = self.current; // The start of the token

        let kind = if c.is_digit(10) {
            self.consume_number()
        } else if c.is_alphabetic() || c == '_' {
            let id = self.consume_id();
            match id.as_str() {
                "immut" | "immutable" => TokenKind::Immut,
                "let" => TokenKind::Let,

                "break" => TokenKind::Break,
                "cont" | "continue" => TokenKind::Continue,
                "else" => TokenKind::Else,
                "if" => TokenKind::If,
                "loop" => TokenKind::Loop,
                "match" => TokenKind::Match,
                "ret" | "return" => TokenKind::Ret,
                "while" => TokenKind::While,

                "enum" | "enumeration" => TokenKind::Enum,
                "fn" | "fun" | "function" => TokenKind::Fun,
                "form" => TokenKind::Form,
                "struct" | "structure" => TokenKind::Struct,
                "use" => TokenKind::Use,
                "C" => TokenKind::CBlock,

                "pri" | "private" => TokenKind::Pri,
                "pub" | "public" => TokenKind::Pub,

                "append" => TokenKind::Append,
                "close" => TokenKind::Close,
                "open" => TokenKind::Open,
                "print" => TokenKind::Print,
                "read" => TokenKind::Read,
                "sort" => TokenKind::Sort,
                "write" => TokenKind::Write,

                "xor" => TokenKind::BitXor,
                "bor" => TokenKind::BitOr,
                "band" => TokenKind::BitAnd,
                "bnot" => TokenKind::BitNot,

                "err" | "error" => TokenKind::Error,
                "false" => TokenKind::False,
                "new" => TokenKind::New,
                "null" | "undefined" => TokenKind::Null,
                "true" => TokenKind::True,

                "bool" | "boolean" => TokenKind::Type(Types::Bool),
                "char" => TokenKind::Type(Types::Char),
                "file" => TokenKind::Type(Types::File),
                "string" => TokenKind::Type(Types::String),

                "i8" => TokenKind::Type(Types::I8),
                "i16" => TokenKind::Type(Types::I16),
                "i32" | "int" | "integer" => TokenKind::Type(Types::I32),
                "i64" => TokenKind::Type(Types::I64),
                "u8" => TokenKind::Type(Types::U8),
                "u16" => TokenKind::Type(Types::U16),
                "u32" => TokenKind::Type(Types::U32),
                "u64" => TokenKind::Type(Types::U64),

                "f32" | "float" | "decimal" => TokenKind::Type(Types::F32),
                "f64" => TokenKind::Type(Types::F64),
                "mat3x3" | "matrix3x3" => TokenKind::Type(Types::Matrix3x3),
                "mat4x4" | "matrix4x4" => TokenKind::Type(Types::Matrix4x4),
                "vec2" | "vector2" => TokenKind::Type(Types::Vector2),
                "vec3" | "vector3" => TokenKind::Type(Types::Vector3),

                _ => TokenKind::Id(id),
            }
        } else {
            self.swallow();
            match c {
                '+' => TokenKind::Plus,
                '-' => self.is_compound('>', TokenKind::Arrow, TokenKind::Minus),
                '*' => TokenKind::Asterisk,
                '/' => TokenKind::Slash,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semicolon,
                '%' => TokenKind::Percent,
                '@' => TokenKind::At,
                '^' => TokenKind::Caret,
                '?' => TokenKind::Huh,
                '=' => self.is_compound('=', TokenKind::EqualEqual, TokenKind::Equal),
                '!' => self.is_compound('=', TokenKind::NotEqual, TokenKind::Bang),
                '|' => self.is_compound('|', TokenKind::TwoPipes, TokenKind::Pipe),
                '&' => self.is_compound('&', TokenKind::TwoAmpersands, TokenKind::Ampersand),
                '~' => TokenKind::Tilde,
                '<' => match self.current_char() {
                    Some('<') => {
                        self.swallow();
                        TokenKind::ShiftLeft
                    }
                    Some('=') => {
                        self.swallow();
                        TokenKind::LessOrEqual
                    }
                    _ => TokenKind::LessThan,
                },
                '>' => match self.current_char() {
                    Some('>') => {
                        self.swallow();
                        TokenKind::ShiftRight
                    }
                    Some('=') => {
                        self.swallow();
                        TokenKind::GreaterOrEqual
                    }
                    _ => TokenKind::GreaterThan,
                },
                '(' => TokenKind::OpeningRound,
                ')' => TokenKind::ClosingRound,
                '{' => TokenKind::OpeningCurly,
                '}' => TokenKind::ClosingCurly,
                '[' => TokenKind::OpeningSquare,
                ']' => TokenKind::ClosingSquare,
                '"' => self.string_token(),
                '\'' => self.char_token(),
                _ => {
                    self.error("Unexpected character");
                    TokenKind::EOF // Placeholder token kind for invalid character
                }
            }
        };

        let end = self.current;
        let literal = self.chars[start..end].iter().collect();
        Some(Token::new(
            kind,
            TextSpan::new(start, end, literal, token_line),
        ))
    }

    fn current_char(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn swallow(&mut self) -> Option<char> {
        if self.current >= self.chars.len() {
            return None;
        }
        let c = self.chars[self.current];
        self.current += 1;
        if c == '\n' {
            self.line += 1;
        }
        Some(c)
    }

    fn consume_number(&mut self) -> TokenKind {
        let mut int_part: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.swallow().unwrap();
                int_part = int_part * 10 + (c as i64 - '0' as i64);
            } else {
                break;
            }
        }
        if self.current_char() == Some('.') {
            let next_char = self.chars.get(self.current + 1).copied();
            if next_char.map_or(false, |c| c.is_digit(10)) {
                self.swallow();
                let mut fraction: f64 = 0.0;
                let mut place = 0.1;
                while let Some(c) = self.current_char() {
                    if c.is_digit(10) {
                        self.swallow();
                        fraction += (c as u8 - b'0') as f64 * place;
                        place *= 0.1;
                    } else {
                        break;
                    }
                }
                return TokenKind::Float(int_part as f64 + fraction);
            }
        }
        TokenKind::Int(int_part)
    }

    fn consume_id(&mut self) -> String {
        let mut id = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphabetic() || c == '_' || c.is_digit(10) {
                self.swallow().unwrap();
                id.push(c);
            } else {
                break;
            }
        }
        id
    }

    fn is_compound(
        &mut self,
        expected: char,
        compound_operator: TokenKind,
        operator: TokenKind,
    ) -> TokenKind {
        if let Some(next) = self.current_char() {
            if next == expected {
                self.swallow();
                compound_operator
            } else {
                operator
            }
        } else {
            operator
        }
    }

    fn string_token(&mut self) -> TokenKind {
        let mut content = String::new();
        while (self.current_char() != Some('"')) && !self.is_eof() {
            content.push(self.consume_escape());
        }
        if self.current_char() != Some('"') {
            self.error("Unterminated string literal");
        } else {
            self.swallow();
        }

        TokenKind::Str(content)
    }

    fn consume_escape(&mut self) -> char {
        let c = match self.current_char() {
            Some('\\') => {
                self.swallow();
                match self.current_char() {
                    Some('n') => {
                        self.swallow();
                        '\n'
                    }
                    Some('t') => {
                        self.swallow();
                        '\t'
                    }
                    Some('r') => {
                        self.swallow();
                        '\r'
                    }
                    Some('\'') => {
                        self.swallow();
                        '\''
                    }
                    Some('\"') => {
                        self.swallow();
                        '\"'
                    }
                    Some('\\') => {
                        self.swallow();
                        '\\'
                    }
                    _ => {
                        self.error("Invalid escape sequence");
                        '\0'
                    }
                }
            }
            Some(ch) => {
                self.swallow();
                ch
            }
            None => {
                self.error("Unexpected EOF inside literal");
                '\0'
            }
        };
        c
    }

    fn char_token(&mut self) -> TokenKind {
        let c = self.consume_escape();
        if self.current_char() != Some('\'') {
            self.error("Unterminated character literal");
        } else {
            self.swallow();
        }
        TokenKind::Char(c)
    }

    fn is_eof(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn error(&mut self, msg: impl Into<String>) {
        let error_position = self.current.min(self.chars.len().saturating_sub(1));
        let mut line_start = error_position;
        while line_start > 0 && self.chars[line_start - 1] != '\n' {
            line_start -= 1
        }

        let mut line_end = line_start;
        while line_end < self.chars.len() && self.chars[line_end] != '\n' {
            line_end += 1
        }

        let src_line = self.chars[line_start..line_end].iter().collect();

        self.errors.push(AsteriError::new(
            ErrorKind::Lexer,
            self.line,
            src_line,
            msg.into(),
        ));
    }

    pub fn take_all(self) -> (Vec<AsteriError>, Vec<AsteriError>, Vec<AsteriError>) {
        (self.errors, self.warnings, self.info)
    }
}
