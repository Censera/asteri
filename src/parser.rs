use crate::{Error, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDeclaration {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub module: Option<String>,
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportItem {
    pub name: String,
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingKind {
    Let,
    Const,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingDeclaration {
    pub kind: BindingKind,
    pub bindings: Vec<Binding>,
    pub value: Option<Vec<TokenKind>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub name: String,
    pub type_tokens: Vec<TokenKind>,
    pub value: Option<Vec<TokenKind>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: Vec<TokenKind>,
    pub parameters: Vec<Parameter>,
    pub flags: Vec<String>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub type_tokens: Vec<TokenKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Binding(BindingDeclaration),
    Return(Option<Expression>),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Integer(String),
    Float(String),
    String(String),
    Character(char),
    Boolean(bool),
    None,
    Identifier(String),
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Member {
        value: Box<Expression>,
        name: String,
    },
    Unary {
        operator: UnaryOperator,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Positive,
    Negative,
    Not,
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Or,
    Xor,
    And,
    BitOr,
    BitXor,
    BitAnd,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    ShiftLeft,
    ShiftRight,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn parse_module(tokens: &[Token]) -> Result<ModuleDeclaration, Error> {
    let mut parser = Parser {
        tokens,
        position: 0,
    };
    parser.expect(TokenKind::Mod)?;
    Ok(ModuleDeclaration {
        name: parser.expect_name()?,
    })
}

pub fn parse_imports(tokens: &[Token]) -> Result<Vec<Import>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_imports()
}

pub fn parse_bindings(tokens: &[Token]) -> Result<Vec<BindingDeclaration>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_bindings()
}

pub fn parse_functions(tokens: &[Token]) -> Result<Vec<FunctionDeclaration>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_functions()
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    fn parse_imports(mut self) -> Result<Vec<Import>, Error> {
        let mut imports = Vec::new();
        while self.peek_kind() == Some(&TokenKind::Use) {
            imports.push(self.parse_import()?);
        }
        Ok(imports)
    }

    fn parse_bindings(mut self) -> Result<Vec<BindingDeclaration>, Error> {
        let mut declarations = Vec::new();
        while matches!(self.peek_kind(), Some(&TokenKind::Let | &TokenKind::Const)) {
            declarations.push(self.parse_binding_declaration()?);
        }
        if self.peek_kind().is_some() {
            return Err(self.error("unexpected token after binding"));
        }
        Ok(declarations)
    }

    fn parse_functions(mut self) -> Result<Vec<FunctionDeclaration>, Error> {
        let mut declarations = Vec::new();
        while self.peek_kind() == Some(&TokenKind::At) || self.peek_kind() == Some(&TokenKind::Fn) {
            declarations.push(self.parse_function()?);
        }
        if self.peek_kind().is_some() {
            return Err(self.error("unexpected token after function"));
        }
        Ok(declarations)
    }

    fn parse_import(&mut self) -> Result<Import, Error> {
        self.expect(TokenKind::Use)?;
        if self.peek_kind() == Some(&TokenKind::OpenBrace) {
            return Ok(Import {
                module: None,
                items: self.parse_items()?,
            });
        }
        let module = self.expect_name()?;
        let items = if self.peek_kind() == Some(&TokenKind::OpenBrace) {
            self.parse_items()?
        } else {
            Vec::new()
        };
        Ok(Import {
            module: Some(module),
            items,
        })
    }

    fn parse_binding_declaration(&mut self) -> Result<BindingDeclaration, Error> {
        let kind = match self.advance() {
            Some(TokenKind::Let) => BindingKind::Let,
            Some(TokenKind::Const) => BindingKind::Const,
            _ => return Err(self.error("expected `let` or `const`")),
        };
        let (bindings, value) = if self.peek_kind() == Some(&TokenKind::OpenBrace) {
            (self.parse_binding_block()?, None)
        } else {
            let bindings = self.parse_binding_names()?;
            self.expect(TokenKind::EqualSign)?;
            let value = self.parse_until_statement_end()?;
            (bindings, Some(value))
        };
        self.consume(TokenKind::Semicolon);
        Ok(BindingDeclaration {
            kind,
            bindings,
            value,
        })
    }

    fn parse_function(&mut self) -> Result<FunctionDeclaration, Error> {
        let flags = self.parse_flags()?;
        self.expect(TokenKind::Fn)?;
        let return_type = if self.peek_kind() == Some(&TokenKind::OpenBracket) {
            self.parse_bracketed_tokens("expected function return type")?
        } else {
            Vec::new()
        };
        let name = self.expect_binding_name()?;
        self.expect(TokenKind::OpenParen)?;
        let parameters = self.parse_parameters()?;
        self.expect(TokenKind::CloseParen)?;
        let body = self.parse_block()?;
        Ok(FunctionDeclaration {
            name,
            return_type,
            parameters,
            flags,
            body,
        })
    }

    fn parse_block(&mut self) -> Result<Block, Error> {
        self.expect(TokenKind::OpenBrace)?;
        let mut statements = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            if self.peek_kind().is_none() {
                return Err(self.error("unterminated block"));
            }
            statements.push(self.parse_statement()?);
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.peek_kind() {
            Some(TokenKind::Let) | Some(TokenKind::Const) => {
                Ok(Statement::Binding(self.parse_binding_declaration()?))
            }
            Some(TokenKind::Return) => {
                self.advance();
                if self.peek_kind() == Some(&TokenKind::Semicolon)
                    || self.peek_kind() == Some(&TokenKind::CloseBrace)
                {
                    self.consume(TokenKind::Semicolon);
                    Ok(Statement::Return(None))
                } else {
                    let expression = self.parse_expression()?;
                    self.consume(TokenKind::Semicolon);
                    Ok(Statement::Return(Some(expression)))
                }
            }
            _ => {
                let expression = self.parse_expression()?;
                self.consume(TokenKind::Semicolon);
                Ok(Statement::Expression(expression))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, minimum_precedence: u8) -> Result<Expression, Error> {
        let mut left = self.parse_unary_expression()?;
        while let Some((operator, precedence)) = self.binary_operator() {
            if precedence < minimum_precedence {
                break;
            }
            self.advance();
            let right = self.parse_binary_expression(precedence + 1)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, Error> {
        let operator = match self.peek_kind() {
            Some(TokenKind::Add) => Some(UnaryOperator::Positive),
            Some(TokenKind::Subtract) => Some(UnaryOperator::Negative),
            Some(TokenKind::Exclamation) => Some(UnaryOperator::Not),
            Some(TokenKind::BitNot) => Some(UnaryOperator::BitNot),
            _ => None,
        };
        if let Some(operator) = operator {
            self.advance();
            return Ok(Expression::Unary {
                operator,
                value: Box::new(self.parse_unary_expression()?),
            });
        }
        self.parse_postfix_expression()
    }

    fn parse_postfix_expression(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_primary_expression()?;
        loop {
            match self.peek_kind() {
                Some(TokenKind::OpenParen) => {
                    self.advance();
                    let mut arguments = Vec::new();
                    if self.peek_kind() != Some(&TokenKind::CloseParen) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if self.peek_kind() != Some(&TokenKind::Comma) {
                                break;
                            }
                            self.advance();
                        }
                    }
                    self.expect(TokenKind::CloseParen)?;
                    expression = Expression::Call {
                        function: Box::new(expression),
                        arguments,
                    };
                }
                Some(TokenKind::Dot) => {
                    self.advance();
                    let name = self.expect_binding_name()?;
                    expression = Expression::Member {
                        value: Box::new(expression),
                        name,
                    };
                }
                _ => break,
            }
        }
        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, Error> {
        match self.advance() {
            Some(TokenKind::Integer(value)) => Ok(Expression::Integer(value)),
            Some(TokenKind::Float(value)) => Ok(Expression::Float(value)),
            Some(TokenKind::String(value)) => Ok(Expression::String(value)),
            Some(TokenKind::Character(value)) => Ok(Expression::Character(value)),
            Some(TokenKind::True) => Ok(Expression::Boolean(true)),
            Some(TokenKind::False) => Ok(Expression::Boolean(false)),
            Some(TokenKind::None) => Ok(Expression::None),
            Some(TokenKind::Identifier(name)) => Ok(Expression::Identifier(name)),
            Some(TokenKind::OpenParen) => {
                let expression = self.parse_expression()?;
                self.expect(TokenKind::CloseParen)?;
                Ok(expression)
            }
            Some(_) => Err(self.error("expected expression")),
            None => Err(self.error("expected expression")),
        }
    }

    fn binary_operator(&self) -> Option<(BinaryOperator, u8)> {
        let value = match self.peek_kind()? {
            TokenKind::Or => (BinaryOperator::Or, 1),
            TokenKind::Xor => (BinaryOperator::Xor, 2),
            TokenKind::And => (BinaryOperator::And, 3),
            TokenKind::BitOr => (BinaryOperator::BitOr, 4),
            TokenKind::BitXor => (BinaryOperator::BitXor, 5),
            TokenKind::BitAnd => (BinaryOperator::BitAnd, 6),
            TokenKind::Equal => (BinaryOperator::Equal, 7),
            TokenKind::NotEqual => (BinaryOperator::NotEqual, 7),
            TokenKind::Greater => (BinaryOperator::Greater, 8),
            TokenKind::GreaterEqual => (BinaryOperator::GreaterEqual, 8),
            TokenKind::Less => (BinaryOperator::Less, 8),
            TokenKind::LessEqual => (BinaryOperator::LessEqual, 8),
            TokenKind::ShiftLeft => (BinaryOperator::ShiftLeft, 9),
            TokenKind::ShiftRight => (BinaryOperator::ShiftRight, 9),
            TokenKind::Add => (BinaryOperator::Add, 10),
            TokenKind::Subtract => (BinaryOperator::Subtract, 10),
            TokenKind::Multiply => (BinaryOperator::Multiply, 11),
            TokenKind::Divide => (BinaryOperator::Divide, 11),
            _ => return None,
        };
        Some(value)
    }

    fn parse_flags(&mut self) -> Result<Vec<String>, Error> {
        let mut flags = Vec::new();
        while self.peek_kind() == Some(&TokenKind::At) {
            self.advance();
            flags.push(self.expect_binding_name()?);
        }
        Ok(flags)
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, Error> {
        let mut parameters = Vec::new();
        if self.peek_kind() == Some(&TokenKind::CloseParen) {
            return Ok(parameters);
        }
        loop {
            if self.peek_kind() == Some(&TokenKind::Ellipsis) {
                self.advance();
                break;
            }
            let name = self.expect_binding_name()?;
            let type_tokens = self.parse_type_tokens(|kind| {
                matches!(
                    kind,
                    TokenKind::Comma | TokenKind::CloseParen | TokenKind::Ellipsis
                )
            });
            if type_tokens.is_empty() {
                return Err(self.error("expected parameter type"));
            }
            parameters.push(Parameter { name, type_tokens });
            if self.peek_kind() == Some(&TokenKind::Ellipsis) {
                self.advance();
                break;
            }
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
                continue;
            }
            break;
        }
        Ok(parameters)
    }

    fn parse_binding_names(&mut self) -> Result<Vec<Binding>, Error> {
        let mut bindings = Vec::new();
        loop {
            let name = self.expect_binding_name()?;
            let type_tokens = self
                .parse_type_tokens(|kind| matches!(kind, TokenKind::Comma | TokenKind::EqualSign));
            bindings.push(Binding {
                name,
                type_tokens,
                value: None,
            });
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
                continue;
            }
            break;
        }
        Ok(bindings)
    }

    fn parse_binding_block(&mut self) -> Result<Vec<Binding>, Error> {
        self.expect(TokenKind::OpenBrace)?;
        let mut bindings = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            let name = self.expect_binding_name()?;
            let type_tokens = self.parse_type_tokens(|kind| {
                matches!(
                    kind,
                    TokenKind::EqualSign | TokenKind::Comma | TokenKind::CloseBrace
                )
            });
            self.expect(TokenKind::EqualSign)?;
            let value = self.parse_until_any(&[TokenKind::Comma, TokenKind::CloseBrace])?;
            bindings.push(Binding {
                name,
                type_tokens,
                value: Some(value),
            });
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in binding block"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(bindings)
    }

    fn parse_bracketed_tokens(&mut self, message: &str) -> Result<Vec<TokenKind>, Error> {
        self.expect(TokenKind::OpenBracket)?;
        let mut tokens = Vec::new();
        let mut depth = 1usize;
        while let Some(kind) = self.peek_kind() {
            match kind {
                TokenKind::OpenBracket => depth += 1,
                TokenKind::CloseBracket => {
                    depth -= 1;
                    if depth == 0 {
                        self.advance();
                        if tokens.is_empty() {
                            return Err(self.error(message));
                        }
                        return Ok(tokens);
                    }
                }
                _ => {}
            }
            tokens.push(self.advance().expect("peeked token must exist"));
        }
        Err(self.error(message))
    }

    fn parse_type_tokens<F>(&mut self, stop: F) -> Vec<TokenKind>
    where
        F: Fn(&TokenKind) -> bool,
    {
        let mut tokens = Vec::new();
        while let Some(kind) = self.peek_kind() {
            if stop(kind) {
                break;
            }
            tokens.push(self.advance().expect("peeked token must exist"));
        }
        tokens
    }

    fn parse_until_statement_end(&mut self) -> Result<Vec<TokenKind>, Error> {
        let mut tokens = Vec::new();
        let mut depth = 0usize;
        while let Some(kind) = self.peek_kind() {
            if depth == 0 && kind == &TokenKind::Semicolon {
                break;
            }
            match kind {
                TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace => depth += 1,
                TokenKind::CloseParen | TokenKind::CloseBracket | TokenKind::CloseBrace => {
                    if depth == 0 {
                        return Err(self.error("unexpected closing delimiter in binding value"));
                    }
                    depth -= 1;
                }
                _ => {}
            }
            tokens.push(self.advance().expect("peeked token must exist"));
        }
        if tokens.is_empty() {
            return Err(self.error("expected binding value"));
        }
        if depth != 0 {
            return Err(self.error("unterminated delimiter in binding value"));
        }
        Ok(tokens)
    }

    fn parse_until_any(&mut self, stops: &[TokenKind]) -> Result<Vec<TokenKind>, Error> {
        let mut tokens = Vec::new();
        let mut depth = 0usize;
        while let Some(kind) = self.peek_kind() {
            if depth == 0 && stops.iter().any(|stop| stop == kind) {
                break;
            }
            match kind {
                TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace => depth += 1,
                TokenKind::CloseParen | TokenKind::CloseBracket | TokenKind::CloseBrace => {
                    if depth == 0 {
                        return Err(self.error("unexpected closing delimiter in binding value"));
                    }
                    depth -= 1;
                }
                _ => {}
            }
            tokens.push(self.advance().expect("peeked token must exist"));
        }
        if tokens.is_empty() {
            return Err(self.error("expected binding value"));
        }
        if depth != 0 {
            return Err(self.error("unterminated delimiter in binding value"));
        }
        Ok(tokens)
    }

    fn consume(&mut self, expected: TokenKind) {
        if self.peek_kind() == Some(&expected) {
            self.advance();
        }
    }

    fn parse_items(&mut self) -> Result<Vec<ImportItem>, Error> {
        self.expect(TokenKind::OpenBrace)?;
        let mut items = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            let name = self.expect_name()?;
            let nested = if self.peek_kind() == Some(&TokenKind::OpenBrace) {
                self.parse_items()?
            } else {
                Vec::new()
            };
            items.push(ImportItem {
                name,
                items: nested,
            });
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in import list"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(items)
    }

    fn expect_name(&mut self) -> Result<String, Error> {
        match self.advance() {
            Some(TokenKind::Identifier(name)) => Ok(name),
            Some(kind) => keyword_name(&kind).ok_or_else(|| self.error("expected identifier")),
            None => Err(self.error("expected identifier")),
        }
    }

    fn expect_binding_name(&mut self) -> Result<String, Error> {
        match self.advance() {
            Some(TokenKind::Identifier(name)) => Ok(name),
            Some(_) => Err(self.error("expected binding name")),
            None => Err(self.error("expected binding name")),
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), Error> {
        if self.peek_kind() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error("unexpected token"))
        }
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        self.tokens.get(self.position).map(Token::kind)
    }

    fn advance(&mut self) -> Option<TokenKind> {
        let kind = self.tokens.get(self.position)?.kind().clone();
        self.position += 1;
        Some(kind)
    }

    fn error(&self, message: &str) -> Error {
        let token = self.tokens.get(self.position);
        let (line, column) = token
            .map(|token| (token.line(), token.column()))
            .unwrap_or_else(|| {
                self.tokens
                    .last()
                    .map(|token| (token.line(), token.column() + 1))
                    .unwrap_or((1, 1))
            });
        Error::Parse {
            line,
            column,
            message: message.to_owned(),
        }
    }
}

fn keyword_name(kind: &TokenKind) -> Option<String> {
    let name = match kind {
        TokenKind::Mod => "mod",
        TokenKind::Use => "use",
        TokenKind::Let => "let",
        TokenKind::Const => "const",
        TokenKind::Fn => "fn",
        TokenKind::Return => "return",
        TokenKind::If => "if",
        TokenKind::Elif => "elif",
        TokenKind::Else => "else",
        TokenKind::Then => "then",
        TokenKind::Break => "break",
        TokenKind::Continue => "continue",
        TokenKind::Loop => "loop",
        TokenKind::While => "while",
        TokenKind::Match => "match",
        TokenKind::For => "for",
        TokenKind::In => "in",
        TokenKind::Enum => "enum",
        TokenKind::Struct => "struct",
        TokenKind::Into => "into",
        TokenKind::Pub => "pub",
        TokenKind::Pri => "pri",
        TokenKind::Type => "type",
        TokenKind::Embed => "embed",
        TokenKind::Macro => "macro",
        TokenKind::True => "true",
        TokenKind::False => "false",
        TokenKind::None => "None",
        _ => return None,
    };
    Some(name.to_owned())
}
