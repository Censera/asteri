use crate::ast::{
    BinaryOp, Expr, MatchArm, MatchPattern, Stmt, StructField, StructMethod, UnaryOp,
};
use crate::error::{AsteriError, ErrorKind};
use crate::lexer::{Token, TokenKind, Types};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<AsteriError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> (Vec<Stmt>, Vec<AsteriError>) {
        let mut stmts = Vec::new();
        while !self.is_eof() {
            match self.parse_stmt() {
                Ok(s) => stmts.push(s),
                Err(e) => {
                    self.errors.push(e);
                    self.recover();
                }
            }
        }
        (stmts, self.errors)
    }

    fn recover(&mut self) {
        while !self.is_eof() {
            match self.kind() {
                TokenKind::Semicolon => {
                    self.advance();
                    return;
                }
                TokenKind::ClosingCurly => {
                    return;
                }
                _ => self.advance(),
            }
        }
    }

    fn error(&self, msg: &str) -> AsteriError {
        let index = self.current.min(self.tokens.len() - 1);
        AsteriError::new(
            ErrorKind::Parser,
            self.tokens[index].span.line,
            "".to_string(),
            msg.to_string(),
        )
    }

    fn parse_stmt(&mut self) -> Result<Stmt, AsteriError> {
        match self.kind() {
            TokenKind::Let => self.parse_let(),
            TokenKind::Immut => self.parse_immut(),
            TokenKind::Print => self.parse_print(),
            TokenKind::Error => self.parse_error(),
            TokenKind::Fun => self.parse_fun(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::Loop => self.parse_loop(),
            TokenKind::Match => self.parse_match(),
            TokenKind::Struct => self.parse_struct(),
            TokenKind::Ret => self.parse_ret(),
            TokenKind::CBlock => self.parse_cblock(),
            TokenKind::Id(_) => {
                let name = self.expect_id()?;
                match self.kind() {
                    TokenKind::Equal => self.parse_assign(name),
                    TokenKind::OpeningRound => self.parse_call(name),
                    _ => Err(self.error("Unexpected Token")),
                }
            }
            _ => Err(self.error("Unexpected Token")),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let name = self.expect_id()?;
        self.expect(TokenKind::Colon)?;
        let tp = self.expect_type()?;
        let value = if matches!(self.kind(), TokenKind::Equal) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Let {
            name,
            tp: Some(tp),
            value,
        })
    }

    fn parse_immut(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let name = self.expect_id()?;
        self.expect(TokenKind::Colon)?;
        let tp = self.expect_type()?;
        let value = if matches!(self.kind(), TokenKind::Equal) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Immut {
            name,
            tp: Some(tp),
            value,
        })
    }

    fn parse_print(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Print(expr))
    }

    fn parse_error(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Error(expr))
    }

    fn parse_fun(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let rt_tp = if matches!(self.kind(), TokenKind::Colon) {
            self.advance();
            Some(self.expect_type()?)
        } else {
            None
        };
        let name = self.expect_id()?;
        self.expect(TokenKind::OpeningRound)?;
        let params = self.parse_params()?;
        self.expect(TokenKind::ClosingRound)?;
        let body = self.parse_block()?;
        Ok(Stmt::Fun {
            rt_tp,
            name,
            params,
            body,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<(String, Types)>, AsteriError> {
        let mut params = Vec::new();
        while !matches!(self.kind(), TokenKind::ClosingRound) {
            let name = self.expect_id()?;
            self.expect(TokenKind::Colon)?;
            let tp = self.expect_type()?;
            params.push((name, tp));
            if matches!(self.kind(), TokenKind::Comma) {
                self.advance();
            }
        }
        Ok(params)
    }

    fn parse_ret(&mut self) -> Result<Stmt, AsteriError> {
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

    fn parse_cblock(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        self.expect(TokenKind::OpeningCurly)?;
        let mut raw_c = String::new();
        let mut depth = 1;
        while depth > 0 {
            match self.kind() {
                TokenKind::EOF => return Err(self.error("Unterminated C Block")),
                TokenKind::OpeningCurly => {
                    depth += 1;
                    raw_c.push('{');
                    self.advance();
                }
                TokenKind::ClosingCurly => {
                    depth -= 1;
                    if depth > 0 {
                        raw_c.push('}');
                    }
                    self.advance();
                }
                _ => {
                    let literal = self.tokens[self.current].span.literal.clone();
                    raw_c.push_str(&literal);
                    raw_c.push(' ');
                    self.advance();
                }
            }
        }
        Ok(Stmt::CBlock(raw_c))
    }

    fn parse_if(&mut self) -> Result<Stmt, AsteriError> {
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

    fn parse_while(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_loop(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let body = self.parse_block()?;
        Ok(Stmt::Loop { body })
    }

    fn parse_match(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let expr = self.parse_expr()?;
        let arms = self.parse_arms()?;
        Ok(Stmt::Match { expr, arms })
    }

    fn parse_arms(&mut self) -> Result<Vec<MatchArm>, AsteriError> {
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
            self.expect(TokenKind::Colon)?;
            let body = self.parse_block()?;
            arms.push(MatchArm { pattern, body });
        }
        self.expect(TokenKind::ClosingCurly)?;
        Ok(arms)
    }

    fn parse_struct(&mut self) -> Result<Stmt, AsteriError> {
        self.advance();
        let name = self.expect_id()?;
        self.expect(TokenKind::OpeningCurly)?;
        let mut methods = Vec::new();
        let mut fields = Vec::new();
        while !matches!(self.kind(), TokenKind::ClosingCurly | TokenKind::EOF) {
            let is_immut = matches!(self.kind(), TokenKind::Immut);
            if is_immut {
                self.advance();
            }
            if matches!(self.kind(), TokenKind::Fun) {
                self.advance();
                let rt_tp = if matches!(self.kind(), TokenKind::Colon) {
                    self.advance();
                    Some(self.expect_type()?)
                } else {
                    None
                };
                let mtd_name = self.expect_id()?;
                self.expect(TokenKind::OpeningRound)?;
                let params = self.parse_params()?;
                self.expect(TokenKind::ClosingRound)?;
                let body = self.parse_block()?;
                methods.push(StructMethod {
                    is_immut,
                    rt_tp,
                    name: mtd_name,
                    params,
                    body,
                });
            } else {
                let fld_name = self.expect_id()?;
                self.expect(TokenKind::Colon)?;
                let tp = self.expect_type()?;
                self.expect(TokenKind::Comma)?;
                fields.push(StructField { name: fld_name, tp });
            }
        }
        self.expect(TokenKind::ClosingCurly)?;
        Ok(Stmt::Struct {
            name,
            fields,
            methods,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, AsteriError> {
        self.expect(TokenKind::OpeningCurly)?;
        let mut stmts = Vec::new();
        while !matches!(self.kind(), TokenKind::ClosingCurly | TokenKind::EOF) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::ClosingCurly)?;
        Ok(stmts)
    }

    fn parse_assign(&mut self, name: String) -> Result<Stmt, AsteriError> {
        self.advance();
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Assign { name, value })
    }

    fn parse_call(&mut self, name: String) -> Result<Stmt, AsteriError> {
        self.advance();
        let mut args = Vec::new();
        while !matches!(self.kind(), TokenKind::ClosingRound | TokenKind::EOF) {
            args.push(self.parse_expr()?);
            if matches!(self.kind(), TokenKind::Comma) {
                self.advance();
            }
        }
        self.expect(TokenKind::ClosingRound)?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Call { name, args })
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
    fn parse_expr(&mut self) -> Result<Expr, AsteriError> {
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
    fn parse_logical_and(&mut self) -> Result<Expr, AsteriError> {
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
    fn parse_additive(&mut self) -> Result<Expr, AsteriError> {
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
    fn parse_comparison(&mut self) -> Result<Expr, AsteriError> {
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
    fn parse_bitwise(&mut self) -> Result<Expr, AsteriError> {
        let mut left = self.parse_shift()?; // 5:
        loop {
            let op = match self.kind() {
                TokenKind::BitOr => BinaryOp::BitOr,
                TokenKind::BitXor => BinaryOp::BitXor,
                TokenKind::BitAnd => BinaryOp::BitAnd,
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
    fn parse_shift(&mut self) -> Result<Expr, AsteriError> {
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
    fn parse_term(&mut self) -> Result<Expr, AsteriError> {
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
    fn parse_unary(&mut self) -> Result<Expr, AsteriError> {
        match self.kind() {
            TokenKind::Ampersand => {
                self.advance();
                Ok(Expr::Reference(Box::new(self.parse_unary()?)))
            }
            TokenKind::BitNot => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::BitNot,
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
                    op: UnaryOp::Minus,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            _ => self.parse_postfix(), // 8:
        }
    }

    // :8
    fn parse_postfix(&mut self) -> Result<Expr, AsteriError> {
        let mut expr = self.parse_primary()?; // 9:
        while matches!(self.kind(), TokenKind::Caret) {
            self.advance();
            expr = Expr::Dereference(Box::new(expr))
        }
        Ok(expr)
    }

    // :9
    fn parse_primary(&mut self) -> Result<Expr, AsteriError> {
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
            TokenKind::Float(f) => {
                let e = Expr::Float(*f);
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
                let name = s.clone();
                self.advance();
                if matches!(self.kind(), TokenKind::OpeningRound) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.kind(), TokenKind::ClosingRound | TokenKind::EOF) {
                        args.push(self.parse_expr()?);
                        if matches!(self.kind(), TokenKind::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(TokenKind::ClosingRound)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Id(name))
                }
            }
            _ => Err(self.error("Expected Expression")), // :Error
        }
    }

    fn kind(&self) -> &TokenKind {
        &self
            .peek()
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
            .kind
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn is_eof(&self) -> bool {
        self.current >= self.tokens.len() - 1
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), AsteriError> {
        if std::mem::discriminant(self.kind()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("Expected {:?}", expected)))
        }
    }

    fn expect_id(&mut self) -> Result<String, AsteriError> {
        if let TokenKind::Id(s) = self.kind() {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else {
            Err(self.error("Expected Identifier"))
        }
    }

    fn expect_type(&mut self) -> Result<Types, AsteriError> {
        // ^ A pointer that can't be null
        if matches!(self.kind(), TokenKind::Caret) {
            self.advance();
            let inner = self.expect_type()?;
            return Ok(Types::Pointer(Box::new(inner)));
        }

        // ? A pointer that can be null
        if matches!(self.kind(), TokenKind::Huh) {
            self.advance();
            self.expect(TokenKind::Caret)?;
            let inner = self.expect_type()?;
            return Ok(Types::OptionPointer(Box::new(inner)));
        }

        // Types
        if let TokenKind::Type(tp) = self.kind() {
            let tp = tp.clone();
            self.advance();
            Ok(tp)
        } else {
            Err(self.error("Expected Type"))
        }
    }
}
