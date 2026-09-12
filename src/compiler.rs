use crate::error::{Error, Stage};
use crate::lexer::{Token, tokenize};
use crate::parser::{
    BindingDeclaration, Import, ModuleDeclaration, parse_bindings, parse_imports, parse_module,
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

    pub fn compile(&self, source: Source) -> Result<(), Error> {
        self.parse_imports(&source)?;
        Err(Error::StageNotImplemented(Stage::Parser))
    }
}
