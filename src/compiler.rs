use crate::error::{Error, Stage};
use crate::lexer::{Token, tokenize};
use crate::parser::{Import, parse_imports};

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

#[derive(Debug, Default)]
pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn tokenize(&self, source: &Source) -> Result<Vec<Token>, Error> {
        tokenize(source.text())
    }

    pub fn parse_imports(&self, source: &Source) -> Result<Vec<Import>, Error> {
        let tokens = self.tokenize(source)?;
        parse_imports(&tokens)
    }

    pub fn compile(&self, source: Source) -> Result<(), Error> {
        self.parse_imports(&source)?;
        Err(Error::StageNotImplemented(Stage::Parser))
    }
}
