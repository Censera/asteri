use crate::error::{Error, Stage};
use crate::lexer::{Token, TokenKind, tokenize};
use crate::parser::{
    BindingDeclaration, FunctionDeclaration, Import, ModuleDeclaration, parse_bindings,
    parse_functions, parse_imports, parse_module,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    name: String,
    text: String,
}

impl Source {
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringChain {
    parts: Vec<TokenKind>,
}

impl StringChain {
    pub fn new(parts: Vec<TokenKind>) -> Self {
        Self { parts }
    }

    pub fn parts(&self) -> &[TokenKind] {
        &self.parts
    }
}

#[derive(Debug, Default)]
pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn tokenize(&self, source: &Source) -> Result<Vec<Token>, Error> {
        tokenize(source.text())
    }

    pub fn parse_module(&self, source: &Source) -> Result<ModuleDeclaration, Error> {
        let tokens = self.tokenize(source)?;
        parse_module(&tokens)
    }

    pub fn parse_imports(&self, source: &Source) -> Result<Vec<Import>, Error> {
        let tokens = self.tokenize(source)?;
        parse_imports(&tokens)
    }

    pub fn parse_bindings(&self, source: &Source) -> Result<Vec<BindingDeclaration>, Error> {
        let tokens = self.tokenize(source)?;
        parse_bindings(&tokens)
    }

    pub fn parse_functions(&self, source: &Source) -> Result<Vec<FunctionDeclaration>, Error> {
        let tokens = self.tokenize(source)?;
        parse_functions(&tokens)
    }

    pub fn parse_string_chain(&self, source: &Source) -> Result<StringChain, Error> {
        let tokens = self.tokenize(source)?;
        parse_string_chain(&tokens)
    }

    pub fn compile(&self, source: Source) -> Result<(), Error> {
        self.parse_imports(&source)?;
        Err(Error::StageNotImplemented(Stage::Parser))
    }
}

fn parse_string_chain(tokens: &[Token]) -> Result<StringChain, Error> {
    if tokens.is_empty() {
        return Err(Error::Parse {
            line: 1,
            column: 1,
            message: "expected string chain".into(),
        });
    }

    let mut parts = Vec::with_capacity(tokens.len());
    for token in tokens {
        if !is_string_chain_part(token.kind()) {
            return Err(Error::Parse {
                line: token.line(),
                column: token.column(),
                message: "expected string-chain value".into(),
            });
        }
        parts.push(token.kind().clone());
    }

    Ok(StringChain::new(parts))
}

fn is_string_chain_part(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Identifier(_)
            | TokenKind::Integer(_)
            | TokenKind::Float(_)
            | TokenKind::String(_)
            | TokenKind::Character(_)
            | TokenKind::True
            | TokenKind::False
            | TokenKind::None
    )
}

#[cfg(test)]
mod tests {
    use super::{Compiler, Source, StringChain};
    use crate::TokenKind;

    #[test]
    fn parses_string_chain() {
        let compiler = Compiler::new();
        let chain = compiler
            .parse_string_chain(&Source::new("test.as", "Hello 42 \"world\" true"))
            .unwrap();

        assert_eq!(
            chain,
            StringChain::new(vec![
                TokenKind::Identifier("Hello".into()),
                TokenKind::Integer("42".into()),
                TokenKind::String("world".into()),
                TokenKind::True,
            ])
        );
    }

    #[test]
    fn rejects_non_value_in_string_chain() {
        let compiler = Compiler::new();
        let error = compiler
            .parse_string_chain(&Source::new("test.as", "hello + world"))
            .unwrap_err();

        assert_eq!(error.stage(), Some(crate::Stage::Parser));
    }
}
