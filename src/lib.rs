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
    use super::{shortcuts, Context};

    #[test]
    fn creates_i32_function() {
        let context = Context::create();
        let module = context.module("test");
        let function = shortcuts::i32_function(&module, "answer");
        let handler = shortcuts::handler(&function, &context);

        shortcuts::return_i32(&handler, 42).unwrap();

        assert_eq!(function.name(), "answer");
        assert!(module.as_ir().contains("define i32 @answer()"));
        assert!(module.as_ir().contains("ret i32 42"));
    }

    #[test]
    fn creates_void_function() {
        let context = Context::create();
        let module = context.module("test");
        let function = shortcuts::void_function(&module, "main");
        let handler = function.handler(&context);

        handler.return_void().unwrap();

        assert!(module.as_ir().contains("define void @main()"));
    }
}
