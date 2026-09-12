use std::{env, fs, process};

use astery::{Compiler, Error, Source};

fn run() -> Result<(), Error> {
    let path = env::args().nth(1).ok_or_else(|| {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "usage: astery <source-file>",
        ))
    })?;

    let text = fs::read_to_string(&path)?;
    Compiler::new().compile(Source::new(path, text))
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}
