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
