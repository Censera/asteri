use crate::ast::{BinaryOp, Expr, MatchArm, MatchPattern, Stmt, UnaryOp};
use crate::lexer::{Token, TokenKind, Types};

#[derive(Debug)]
pub struct ParseError {
    pub content: String,
    pub line: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Parser] {}", self.content)
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
            TokenKind::Const => self.parse_const(),
            TokenKind::Print => self.parse_print(),
            TokenKind::Error => self.parse_error(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::Loop => self.parse_loop(),
            TokenKind::Match => self.parse_match(),
            TokenKind::Ret => self.parse_ret(),
            TokenKind::Id(_) => {
                if matches!(self.peek(), Some(TokenKind::Equal)) {
                    let name = self.expect_id()?;
                    self.advance();
                    let value = self.parse_expr()?;
                    self.expect(TokenKind::Semicolon)?;
                    Ok(Stmt::Assign { name, value })
                } else {
                    Err(self.error("Unexpected Token"))
                }
            }
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

    fn parse_const(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name = self.expect_id()?;
        self.expect(TokenKind::Colon)?;
        let tp = self.expect_type()?;
        self.expect(TokenKind::Equal)?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Const { name, tp, value })
    }

    fn parse_assign(&mut self, name: String) -> Result<Stmt, ParseError> {
        self.advance();
        self.expect(TokenKind::Equal);
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Assign { name, value })
    }

    fn parse_print(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Print(expr))
    }

    fn parse_error(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Error(expr))
    }

    fn parse_ret(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        if matches!(self.kind(), TokenKind::Semicolon) {
            self.advance();
            Ok(Stmt::Ret(None))
        } else {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Stmt::Ret(Some(expr)))
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        let else_branch = if matches!(self.kind(), TokenKind::Else) {
            self.advance();
            if matches!(self.kind(), TokenKind::If) {
                Some(Box::new(self.parse_if()?))
            } else {
                Some(Box::new(Stmt::Block(self.parse_block()?)))
            }
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            body,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_loop(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let body = self.parse_block()?;
        Ok(Stmt::Loop { body })
    }

    fn parse_match(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let expr = self.parse_expr()?;
        let arms = self.parse_arms()?;
        Ok(Stmt::Match { expr, arms })
    }

    fn parse_arms(&mut self) -> Result<Vec<MatchArm>, ParseError> {
        self.expect(TokenKind::OpeningCurly)?;
        let mut arms = Vec::new();
        while !matches!(self.kind(), TokenKind::ClosingCurly | TokenKind::EOF) {
            let pattern = match self.kind() {
                TokenKind::Id(s) if s == "_" => {
                    self.advance();
                    MatchPattern::Default
                }
                _ => MatchPattern::Expr(self.parse_expr()?),
            };
        }
        self.expect(TokenKind::ClosingCurly)?;
        Ok(arms)
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(TokenKind::OpeningCurly)?;
        let mut stmts = Vec::new();
        while !matches!(self.kind(), TokenKind::ClosingCurly | TokenKind::EOF) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::ClosingCurly)?;
        Ok(stmts)
    }

    // :>   parse_expr          1:
    // :1   parse_logical_and   2:
    // :2   parse_additive      3:
    // :3   parse_comparison    4:
    // :4   parse_bitwise       5:
    // :5   parse_shift         6:
    // :6   parse_term          7:
    // :7   parse_unary         8:
    // :8   parse_primary       Error:

    // :>
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_logical_and()?; // 1:
        loop {
            let op = match self.kind() {
                TokenKind::TwoPipes => BinaryOp::LogicOr,
                _ => break,
            };
            self.advance();
            let right = self.parse_logical_and()?; // 1:
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // :1
    fn parse_logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?; // 2:
        loop {
            let op = match self.kind() {
                TokenKind::TwoAmpersands => BinaryOp::LogicAnd,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?; // 2:
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // :2
    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?; // 3:
        while matches!(self.kind(), TokenKind::Plus | TokenKind::Minus) {
            let op = match self.kind() {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?; // 3:
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // :3
    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_bitwise()?; // 4:
        loop {
            let op = match self.kind() {
                TokenKind::EqualEqual => BinaryOp::Eql,
                TokenKind::NotEqual => BinaryOp::Neq,
                TokenKind::LessThan => BinaryOp::LessTh,
                TokenKind::GreaterThan => BinaryOp::GreaTh,
                TokenKind::LessOrEqual => BinaryOp::LessOr,
                TokenKind::GreaterOrEqual => BinaryOp::GreaOr,
                _ => break,
            };
            self.advance();
            let right = self.parse_bitwise()?; // 4:
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // :4
    fn parse_bitwise(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_shift()?; // 5:
        loop {
            let op = match self.kind() {
                TokenKind::Pipe => BinaryOp::BitOr,
                TokenKind::Caret => BinaryOp::BitXor,
                TokenKind::Ampersand => BinaryOp::BitAnd,
                _ => break,
            };
            self.advance();
            let right = self.parse_shift()?; // 5:
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // :5
    fn parse_shift(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?; // 6:
        loop {
            let op = match self.kind() {
                TokenKind::ShiftLeft => BinaryOp::ShiftLeft,
                TokenKind::ShiftRight => BinaryOp::ShiftRight,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?; // 6:
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // :6
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?; // 7:
        while matches!(self.kind(), TokenKind::Asterisk | TokenKind::Slash) {
            let op = match self.kind() {
                TokenKind::Asterisk => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?; // 7:
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    // :7
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.kind() {
            TokenKind::Tilde => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            TokenKind::Bang => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            TokenKind::Minus => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Mns,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            _ => self.parse_primary(), // 8:
        }
    }

    // :8
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.kind() {
            TokenKind::OpeningRound => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::ClosingRound)?;
                Ok(expr)
            }
            TokenKind::Int(n) => {
                let e = Expr::Int(*n);
                self.advance();
                Ok(e)
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            TokenKind::Str(string) => {
                let e = Expr::Str(string.clone());
                self.advance();
                Ok(e)
            }
            TokenKind::Id(s) => {
                let e = Expr::Id(s.clone());
                self.advance();
                Ok(e)
            }
            _ => Err(self.error("Expected Expression")), // :Error
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

    fn peek(&mut self) -> Option<&TokenKind> {
        self.tokens.get(self.current + 1).map(|n| &n.kind)
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
            self.advance();
            Ok(s)
        } else {
            Err(self.error("Expected Identifier"))
        }
    }

    fn expect_type(&mut self) -> Result<Types, ParseError> {
        if let TokenKind::Type(tp) = self.kind() {
            let tp = tp.clone();
            self.advance();
            Ok(tp)
        } else {
            Err(self.error("Expected Type"))
        }
    }

    fn error(&self, content: &str) -> ParseError {
        ParseError {
            content: content.to_string(),
            line: self.tokens[self.current].span.line,
        }
    }
}
