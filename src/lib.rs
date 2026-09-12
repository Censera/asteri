mod context;
mod function;
mod handler;
mod module;
pub mod shortcuts;

pub use context::Context;
pub use function::Function;
pub use handler::Handler;
pub use module::Module;

#[cfg(test)]
mod tests {
    use super::{Context, shortcuts};

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
}
