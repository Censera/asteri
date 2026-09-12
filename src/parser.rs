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

pub fn parse_module(tokens: &[Token]) -> Result<ModuleDeclaration, Error> {
    let mut parser = Parser { tokens, position: 0 };
    parser.expect(TokenKind::Mod)?;
    Ok(ModuleDeclaration {
        name: parser.expect_identifier()?,
    })
}

pub fn parse_imports(tokens: &[Token]) -> Result<Vec<Import>, Error> {
    Parser { tokens, position: 0 }.parse_imports()
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

    fn parse_import(&mut self) -> Result<Import, Error> {
        self.expect(TokenKind::Use)?;
        let module = self.expect_identifier()?;

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

    fn parse_items(&mut self) -> Result<Vec<ImportItem>, Error> {
        self.expect(TokenKind::OpenBrace)?;

        let mut items = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            let name = self.expect_identifier()?;
            let nested = if self.peek_kind() == Some(&TokenKind::OpenBrace) {
                self.parse_items()?
            } else {
                Vec::new()
            };
            items.push(ImportItem { name, items: nested });

            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in import list"));
            }
        }

        self.expect(TokenKind::CloseBrace)?;
        Ok(items)
    }

    fn expect_identifier(&mut self) -> Result<String, Error> {
        match self.peek_kind() {
            Some(TokenKind::Identifier(_)) => match self.advance() {
                Some(TokenKind::Identifier(name)) => Ok(name),
                _ => unreachable!(),
            },
            _ => Err(self.error("expected identifier")),
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
