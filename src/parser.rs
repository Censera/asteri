use crate::ast::{Expr, Stmt};
use crate::lexer::{Token, TokenKind, Types};

#[derive(Debug)]
pub struct ParseError {
    pub content: String,
    pub line: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse Error: {}", self.content)
    }
}

impl std::error::Error for ParseError {}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while !self.is_eof() {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.kind() {
            TokenKind::Let => self.parse_let(),
            TokenKind::Print => self.parse_print(),
            TokenKind::Ret => self.parse_return(),
            _ => Err(self.error("Unexpected Token")),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name = self.expect_id()?;
        self.expect(TokenKind::Colon)?;
        let tp = self.expect_type()?;
        self.expect(TokenKind::Equal)?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Let { name, tp, value })
    }

    fn parse_print(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Print(expr))
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        match self.kind() {
            TokenKind::Int(n) => {
                let ex = Expr::Int(*n);
                self.advance();
                Ok(ex)
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            TokenKind::Id(s) => {
                let ex = Expr::Id(s.clone());
                self.advance();
                Ok(ex)
            }
            _ => Err(self.error("Expected Expression")),
        }
    }

    fn kind(&self) -> &TokenKind {
        &self.tokens[self.current].kind
    }

    fn advance(&mut self) {
        if !self.is_eof() {
            self.current += 1;
        }
    }

    fn is_eof(&self) -> bool {
        matches!(self.kind(), TokenKind::EOF)
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        if std::mem::discriminant(self.kind()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("Expected {:?}", expected)))
        }
    }

    fn expect_id(&mut self) -> Result<String, ParseError> {
        if let TokenKind::Id(s) = self.kind() {
            let s = s.clone();
            Ok(s)
        } else {
            Err(self.error("Expected Indetifier"))
        }
    }

    fn expect_type(&mut self) -> Result<Types, ParseError> {
        if let TokenKind::Type(tp) = self.kind() {
            let tp = match tp {
                Types::I8 => Types::I8,
                Types::I16 => Types::I16,
                Types::I32 => Types::I32,
                Types::I64 => Types::I64,
                Types::U8 => Types::U8,
                Types::U16 => Types::U16,
                Types::U32 => Types::U32,
                Types::U64 => Types::U64,
                Types::F32 => Types::F32,
                Types::F64 => Types::F64,
                Types::Bool => Types::Bool,
                Types::String => Types::String,
                Types::Char => Types::Char,
                Types::File => Types::File,
                Types::Vector2 => Types::Vector2,
                Types::Vector3 => Types::Vector3,
                Types::Matrix3x3 => Types::Matrix3x3,
                Types::Matrix4x4 => Types::Matrix4x4,
            };
            self.advance();
            Ok(tp)
        } else {
            Err(self.error("Expected Type"))
        }
    }

    fn error(&self, content: &str) -> ParseError {
        ParseError {
            content: content.to_string(),
            line: 0,
        }
    }
}
