mod compiler;
mod context;
mod error;
mod function;
mod handler;
mod module;
pub mod shortcuts;

pub use compiler::{Compiler, Source};
pub use context::Context;
pub use error::{Error, Stage};
pub use function::Function;
pub use handler::Handler;
pub use module::Module;

#[cfg(test)]
mod tests {
    use super::{shortcuts, Compiler, Context, Error, Source, Stage};

    #[test]
    fn creates_i32_function() {
        let context = Context::create();
        let module = context.module("test").unwrap();
        let function = shortcuts::i32_function(&module, "answer").unwrap();
        let handler = shortcuts::handler(&function, &context).unwrap();

        shortcuts::return_i32(&handler, 42).unwrap();

        assert_eq!(function.name(), "answer");
        assert!(module.as_ir().contains("define i32 @answer()"));
        assert!(module.as_ir().contains("ret i32 42"));
    }

    #[test]
    fn creates_void_function() {
        let context = Context::create();
        let module = context.module("test").unwrap();
        let function = shortcuts::void_function(&module, "main").unwrap();
        let handler = function.handler(&context).unwrap();

        handler.return_void().unwrap();

        assert!(module.as_ir().contains("define void @main()"));
    }

    #[test]
    fn compiler_reports_unimplemented_stage() {
        let compiler = Compiler::new();
        let error = compiler.compile(Source::new("test.astery", "fn main() {}"));

        assert_eq!(error.unwrap_err().stage(), Some(Stage::Lexer));
    }

    #[test]
    fn source_owns_name_and_text() {
        let source = Source::new("test.astery", "fn main() {}");

        assert_eq!(source.name(), "test.astery");
        assert_eq!(source.text(), "fn main() {}");
    }

    #[test]
    fn io_errors_are_explicit() {
        let error = Error::from(std::io::Error::other("test"));

        assert_eq!(error.stage(), None);
    }
}
