use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Lexer,
    Parser,
    Semantic,
    Backend,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    StageNotImplemented(Stage),
}

impl Error {
    pub fn stage(&self) -> Option<Stage> {
        match self {
            Self::Io(_) => None,
            Self::StageNotImplemented(stage) => Some(*stage),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::StageNotImplemented(stage) => {
                write!(f, "compiler stage is not implemented: {stage:?}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
