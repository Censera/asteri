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
                TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace => {
                    depth += 1;
                }
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
                TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace => {
                    depth += 1;
                }
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
