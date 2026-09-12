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
    Print,
    Eprint,
    Sizeof,
    Length,
    Format,
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
    Chain,
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
            if self.skip_whitespace_and_comments() {
                continue;
            }

            let line = self.line;
            let column = self.column;
            let token = self.next_token(line, column)?;
            tokens.push(token);
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
            b'@' => Ok(Token::new(TokenKind::At, line, column)),
            b';' => Ok(Token::new(TokenKind::Semicolon, line, column)),
            b',' => Ok(Token::new(TokenKind::Comma, line, column)),
            b'(' => Ok(Token::new(TokenKind::OpenParen, line, column)),
            b')' => Ok(Token::new(TokenKind::CloseParen, line, column)),
            b'{' => Ok(Token::new(TokenKind::OpenBrace, line, column)),
            b'}' => Ok(Token::new(TokenKind::CloseBrace, line, column)),
            b'[' => Ok(Token::new(TokenKind::OpenBracket, line, column)),
            b']' => Ok(Token::new(TokenKind::CloseBracket, line, column)),
            b'<' => self.operator(TokenKind::Less, TokenKind::LessEqual, b'=', line, column),
            b'>' => self.operator(TokenKind::Greater, TokenKind::GreaterEqual, b'=', line, column),
            b'+' => self.double_operator(TokenKind::Increment, TokenKind::AddAssign, TokenKind::Add, b'+', b'=', line, column),
            b'-' => {
                if self.matches(b'>') {
                    Ok(Token::new(TokenKind::Arrow, line, column))
                } else if self.matches(b'=') {
                    Ok(Token::new(TokenKind::SubAssign, line, column))
                } else if self.matches(b'-') {
                    Ok(Token::new(TokenKind::Decrement, line, column))
                } else {
                    Ok(Token::new(TokenKind::Subtract, line, column))
                }
            }
            b'*' => self.double_operator(TokenKind::MulAssign, TokenKind::MulAssign, TokenKind::Multiply, b'*', b'=', line, column),
            b'/' => self.double_operator(TokenKind::DivAssign, TokenKind::DivAssign, TokenKind::Divide, b'/', b'=', line, column),
            b'=' => {
                if self.matches(b'=') {
                    Ok(Token::new(TokenKind::Equal, line, column))
                } else {
                    Ok(Token::new(TokenKind::EqualSign, line, column))
                }
            }
            b'!' => {
                if self.matches(b'=') {
                    Ok(Token::new(TokenKind::NotEqual, line, column))
                } else {
                    Ok(Token::new(TokenKind::Exclamation, line, column))
                }
            }
            b'&' => {
                if self.matches(b'&') {
                    Ok(Token::new(TokenKind::And, line, column))
                } else {
                    Ok(Token::new(TokenKind::Ampersand, line, column))
                }
            }
            b'|' => {
                if self.matches(b'|') {
                    Ok(Token::new(TokenKind::Or, line, column))
                } else {
                    Err(self.lex_error(line, column, "unexpected `|`"))
                }
            }
            b'^' => {
                if self.matches(b'^') {
                    Ok(Token::new(TokenKind::Xor, line, column))
                } else {
                    Ok(Token::new(TokenKind::Caret, line, column))
                }
            }
            b':' => self.colon_operator(line, column),
            b'.' => self.dot_operator(line, column),
            b'?' => Ok(Token::new(TokenKind::Question, line, column)),
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
        while let Some(byte) = self.peek() {
            if byte.is_ascii_digit() {
                value.push(self.advance().expect("peeked byte must exist") as char);
            } else {
                break;
            }
        }

        if self.peek() == Some(b'.') && self.peek_next().is_some_and(|byte| byte.is_ascii_digit()) {
            value.push(self.advance().expect("peeked byte must exist") as char);
            while let Some(byte) = self.peek() {
                if byte.is_ascii_digit() {
                    value.push(self.advance().expect("peeked byte must exist") as char);
                } else {
                    break;
                }
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
        match self.peek() {
            Some(b'\\') => {
                self.advance();
                let value = self.escape(line, column)?;
                if !self.matches(b'\'') {
                    return Err(self.lex_error(line, column, "unterminated character literal"));
                }
                Ok(Token::new(TokenKind::Character(value), line, column))
            }
            Some(b'\'') => {
                self.advance();
                Ok(Token::new(TokenKind::Character('\''), line, column))
            }
            Some(byte) if is_identifier_start(byte) => {
                let mut value = String::new();
                while let Some(byte) = self.peek() {
                    if is_identifier_continue(byte) {
                        value.push(self.advance().expect("peeked byte must exist") as char);
                    } else {
                        break;
                    }
                }
                Ok(Token::new(TokenKind::Label(value), line, column))
            }
            Some(byte) => {
                self.advance();
                if self.matches(b'\'') {
                    Ok(Token::new(TokenKind::Character(byte as char), line, column))
                } else {
                    Err(self.lex_error(line, column, "invalid character literal"))
                }
            }
            None => Err(self.lex_error(line, column, "unterminated character literal")),
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
            Some(byte) => Err(self.lex_error(line, column, &format!("unknown escape `\\{}`", byte as char))),
            None => Err(self.lex_error(line, column, "unterminated escape sequence")),
        }
    }

    fn operator(
        &mut self,
        single: TokenKind,
        paired: TokenKind,
        expected: u8,
        line: usize,
        column: usize,
    ) -> Result<Token, crate::Error> {
        if self.matches(expected) {
            Ok(Token::new(paired, line, column))
        } else {
            Ok(Token::new(single, line, column))
        }
    }

    fn double_operator(
        &mut self,
        doubled: TokenKind,
        assigned: TokenKind,
        single: TokenKind,
        doubled_byte: u8,
        assign_byte: u8,
        line: usize,
        column: usize,
    ) -> Result<Token, crate::Error> {
        if self.matches(doubled_byte) {
            Ok(Token::new(doubled, line, column))
        } else if self.matches(assign_byte) {
            Ok(Token::new(assigned, line, column))
        } else {
            Ok(Token::new(single, line, column))
        }
    }

    fn colon_operator(&mut self, line: usize, column: usize) -> Result<Token, crate::Error> {
        let token = match self.peek() {
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
        Ok(Token::new(token, line, column))
    }

    fn dot_operator(&mut self, line: usize, column: usize) -> Result<Token, crate::Error> {
        if self.matches(b'.') {
            if self.matches(b'.') {
                Ok(Token::new(TokenKind::Ellipsis, line, column))
            } else {
                Ok(Token::new(TokenKind::Chain, line, column))
            }
        } else {
            Ok(Token::new(TokenKind::Dot, line, column))
        }
    }

    fn skip_whitespace_and_comments(&mut self) -> bool {
        let mut skipped = false;

        loop {
            while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
                skipped = true;
                self.advance();
            }

            if self.peek() == Some(b'/') && self.peek_next() == Some(b'/') {
                skipped = true;
                self.advance();
                self.advance();
                while let Some(byte) = self.peek() {
                    self.advance();
                    if byte == b'\n' {
                        break;
                    }
                }
                continue;
            }

            return skipped;
        }
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
        "print" => TokenKind::Print,
        "eprint" => TokenKind::Eprint,
        "sizeof" => TokenKind::Sizeof,
        "length" => TokenKind::Length,
        "format" => TokenKind::Format,
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
