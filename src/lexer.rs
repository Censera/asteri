#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
}

impl Token {
    fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier(String),
    Integer(String),
    Float(String),
    String(String),
    Character(char),
    Label(String),
    Mod,
    Use,
    Let,
    Const,
    Fn,
    Return,
    If,
    Elif,
    Else,
    Then,
    Break,
    Continue,
    Loop,
    While,
    Match,
    For,
    In,
    Enum,
    Struct,
    Into,
    Pub,
    Pri,
    Type,
    Embed,
    Macro,
    True,
    False,
    None,
    At,
    Arrow,
    And,
    Or,
    Xor,
    Not,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    Equal,
    GreaterEqual,
    LessEqual,
    NotEqual,
    Ellipsis,
    Range,
    RangeInclusive,
    Dot,
    Add,
    Subtract,
    Multiply,
    Divide,
    EqualSign,
    Greater,
    Less,
    Ampersand,
    Question,
    Caret,
    Exclamation,
    Colon,
    Semicolon,
    Comma,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenAngle,
    CloseAngle,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, crate::Error> {
    Lexer::new(source).tokenize()
}

struct Lexer<'src> {
    source: &'src [u8],
    position: usize,
    line: usize,
    column: usize,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source: source.as_bytes(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, crate::Error> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.is_at_end() {
                break;
            }
            let line = self.line;
            let column = self.column;
            tokens.push(self.next_token(line, column)?);
        }
        Ok(tokens)
    }

    fn next_token(&mut self, line: usize, column: usize) -> Result<Token, crate::Error> {
        let byte = self.advance().expect("lexer position must be valid");

        if is_identifier_start(byte) {
            return Ok(self.identifier(byte, line, column));
        }
        if byte.is_ascii_digit() {
            return self.number(byte, line, column);
        }

        match byte {
            b'"' => self.string(line, column),
            b'\'' => self.apostrophe_token(line, column),
            b'@' => Ok(self.simple(TokenKind::At, line, column)),
            b';' => Ok(self.simple(TokenKind::Semicolon, line, column)),
            b',' => Ok(self.simple(TokenKind::Comma, line, column)),
            b'(' => Ok(self.simple(TokenKind::OpenParen, line, column)),
            b')' => Ok(self.simple(TokenKind::CloseParen, line, column)),
            b'{' => Ok(self.simple(TokenKind::OpenBrace, line, column)),
            b'}' => Ok(self.simple(TokenKind::CloseBrace, line, column)),
            b'[' => Ok(self.simple(TokenKind::OpenBracket, line, column)),
            b']' => Ok(self.simple(TokenKind::CloseBracket, line, column)),
            b'<' => {
                if self.matches(b'<') {
                    Ok(self.simple(TokenKind::ShiftLeft, line, column))
                } else if self.matches(b'=') {
                    Ok(self.simple(TokenKind::LessEqual, line, column))
                } else {
                    Ok(self.simple(TokenKind::Less, line, column))
                }
            }
            b'>' => {
                if self.matches(b'>') {
                    Ok(self.simple(TokenKind::ShiftRight, line, column))
                } else if self.matches(b'=') {
                    Ok(self.simple(TokenKind::GreaterEqual, line, column))
                } else {
                    Ok(self.simple(TokenKind::Greater, line, column))
                }
            }
            b'+' => {
                if self.matches(b'+') {
                    Ok(self.simple(TokenKind::Increment, line, column))
                } else if self.matches(b'=') {
                    Ok(self.simple(TokenKind::AddAssign, line, column))
                } else {
                    Ok(self.simple(TokenKind::Add, line, column))
                }
            }
            b'-' => {
                if self.matches(b'>') {
                    Ok(self.simple(TokenKind::Arrow, line, column))
                } else if self.matches(b'-') {
                    Ok(self.simple(TokenKind::Decrement, line, column))
                } else if self.matches(b'=') {
                    Ok(self.simple(TokenKind::SubAssign, line, column))
                } else {
                    Ok(self.simple(TokenKind::Subtract, line, column))
                }
            }
            b'*' => {
                if self.matches(b'=') {
                    Ok(self.simple(TokenKind::MulAssign, line, column))
                } else {
                    Ok(self.simple(TokenKind::Multiply, line, column))
                }
            }
            b'/' => {
                if self.matches(b'=') {
                    Ok(self.simple(TokenKind::DivAssign, line, column))
                } else {
                    Ok(self.simple(TokenKind::Divide, line, column))
                }
            }
            b'=' => {
                if self.matches(b'=') {
                    Ok(self.simple(TokenKind::Equal, line, column))
                } else {
                    Ok(self.simple(TokenKind::EqualSign, line, column))
                }
            }
            b'!' => {
                if self.matches(b'=') {
                    Ok(self.simple(TokenKind::NotEqual, line, column))
                } else {
                    Ok(self.simple(TokenKind::Exclamation, line, column))
                }
            }
            b'&' => {
                if self.matches(b'&') {
                    Ok(self.simple(TokenKind::And, line, column))
                } else {
                    Ok(self.simple(TokenKind::Ampersand, line, column))
                }
            }
            b'|' => {
                if self.matches(b'|') {
                    Ok(self.simple(TokenKind::Or, line, column))
                } else {
                    Err(self.lex_error(line, column, "unexpected `|`"))
                }
            }
            b'^' => {
                if self.matches(b'^') {
                    Ok(self.simple(TokenKind::Xor, line, column))
                } else {
                    Ok(self.simple(TokenKind::Caret, line, column))
                }
            }
            b':' => {
                let kind = match self.peek() {
                    Some(b'&') => {
                        self.advance();
                        TokenKind::BitAnd
                    }
                    Some(b'|') => {
                        self.advance();
                        TokenKind::BitOr
                    }
                    Some(b'^') => {
                        self.advance();
                        TokenKind::BitXor
                    }
                    Some(b'<') => {
                        self.advance();
                        TokenKind::BitNot
                    }
                    _ => TokenKind::Colon,
                };
                Ok(self.simple(kind, line, column))
            }
            b'.' => {
                if self.matches(b'.') {
                    if self.matches(b'.') {
                        Ok(self.simple(TokenKind::Ellipsis, line, column))
                    } else if self.matches(b'=') {
                        Ok(self.simple(TokenKind::RangeInclusive, line, column))
                    } else {
                        Ok(self.simple(TokenKind::Range, line, column))
                    }
                } else {
                    Ok(self.simple(TokenKind::Dot, line, column))
                }
            }
            b'?' => Ok(self.simple(TokenKind::Question, line, column)),
            _ => Err(self.lex_error(line, column, "unexpected character")),
        }
    }

    fn identifier(&mut self, first: u8, line: usize, column: usize) -> Token {
        let mut value = String::from(first as char);
        while let Some(byte) = self.peek() {
            if is_identifier_continue(byte) {
                value.push(self.advance().expect("peeked byte must exist") as char);
            } else {
                break;
            }
        }
        Token::new(keyword(&value), line, column)
    }

    fn number(&mut self, first: u8, line: usize, column: usize) -> Result<Token, crate::Error> {
        let mut value = String::from(first as char);
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            value.push(self.advance().expect("peeked byte must exist") as char);
        }
        if self.peek() == Some(b'.') && self.peek_next().is_some_and(|byte| byte.is_ascii_digit()) {
            value.push(self.advance().expect("peeked byte must exist") as char);
            while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                value.push(self.advance().expect("peeked byte must exist") as char);
            }
            return Ok(Token::new(TokenKind::Float(value), line, column));
        }
        Ok(Token::new(TokenKind::Integer(value), line, column))
    }

    fn string(&mut self, line: usize, column: usize) -> Result<Token, crate::Error> {
        let mut value = String::new();
        while let Some(byte) = self.advance() {
            match byte {
                b'"' => return Ok(Token::new(TokenKind::String(value), line, column)),
                b'\\' => value.push(self.escape(line, column)?),
                b'\n' => return Err(self.lex_error(line, column, "unterminated string literal")),
                byte => value.push(byte as char),
            }
        }
        Err(self.lex_error(line, column, "unterminated string literal"))
    }

    fn apostrophe_token(&mut self, line: usize, column: usize) -> Result<Token, crate::Error> {
        if self.peek() == Some(b'\\') {
            self.advance();
            let value = self.escape(line, column)?;
            if !self.matches(b'\'') {
                return Err(self.lex_error(line, column, "unterminated character literal"));
            }
            return Ok(Token::new(TokenKind::Character(value), line, column));
        }

        let Some(first) = self.peek() else {
            return Err(self.lex_error(line, column, "unterminated character literal"));
        };

        if is_identifier_start(first) {
            let mut value = String::new();
            while self.peek().is_some_and(is_identifier_continue) {
                value.push(self.advance().expect("peeked byte must exist") as char);
            }
            if self.matches(b'\'') {
                if value.chars().count() == 1 {
                    return Ok(Token::new(
                        TokenKind::Character(value.chars().next().unwrap()),
                        line,
                        column,
                    ));
                }
                return Err(self.lex_error(
                    line,
                    column,
                    "character literal must contain one character",
                ));
            }
            return Ok(Token::new(TokenKind::Label(value), line, column));
        }

        let value = self.advance().expect("peeked byte must exist") as char;
        if self.matches(b'\'') {
            Ok(Token::new(TokenKind::Character(value), line, column))
        } else {
            Err(self.lex_error(line, column, "invalid character literal"))
        }
    }

    fn escape(&mut self, line: usize, column: usize) -> Result<char, crate::Error> {
        match self.advance() {
            Some(b'n') => Ok('\n'),
            Some(b'r') => Ok('\r'),
            Some(b't') => Ok('\t'),
            Some(b'0') => Ok('\0'),
            Some(b'\\') => Ok('\\'),
            Some(b'"') => Ok('"'),
            Some(b'\'') => Ok('\''),
            Some(byte) => Err(self.lex_error(
                line,
                column,
                &format!("unknown escape `\\{}`", byte as char),
            )),
            None => Err(self.lex_error(line, column, "unterminated escape sequence")),
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
                self.advance();
            }
            if self.peek() == Some(b'/') && self.peek_next() == Some(b'/') {
                self.advance();
                self.advance();
                while let Some(byte) = self.advance() {
                    if byte == b'\n' {
                        break;
                    }
                }
                continue;
            }
            break;
        }
    }

    fn simple(&self, kind: TokenKind, line: usize, column: usize) -> Token {
        Token::new(kind, line, column)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.position).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.source.get(self.position + 1).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.position += 1;
        if byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(byte)
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn lex_error(&self, line: usize, column: usize, message: &str) -> crate::Error {
        crate::Error::Lex {
            line,
            column,
            message: message.to_owned(),
        }
    }
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn keyword(value: &str) -> TokenKind {
    match value {
        "mod" => TokenKind::Mod,
        "use" => TokenKind::Use,
        "let" => TokenKind::Let,
        "const" => TokenKind::Const,
        "fn" => TokenKind::Fn,
        "return" => TokenKind::Return,
        "if" => TokenKind::If,
        "elif" => TokenKind::Elif,
        "else" => TokenKind::Else,
        "then" => TokenKind::Then,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "loop" => TokenKind::Loop,
        "while" => TokenKind::While,
        "match" => TokenKind::Match,
        "for" => TokenKind::For,
        "in" => TokenKind::In,
        "enum" => TokenKind::Enum,
        "struct" => TokenKind::Struct,
        "into" => TokenKind::Into,
        "pub" => TokenKind::Pub,
        "pri" => TokenKind::Pri,
        "type" => TokenKind::Type,
        "embed" => TokenKind::Embed,
        "macro" => TokenKind::Macro,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "None" => TokenKind::None,
        _ => TokenKind::Identifier(value.to_owned()),
    }
}
