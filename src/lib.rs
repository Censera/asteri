mod compiler;
mod context;
mod error;
mod function;
mod handler;
mod lexer;
mod module;
mod parser;
pub mod shortcuts;

pub use compiler::{Compiler, Source};
pub use context::Context;
pub use error::{Error, Stage};
pub use function::Function;
pub use handler::Handler;
pub use lexer::{Token, TokenKind};
pub use module::Module;
pub use parser::{Binding, BindingDeclaration, BindingKind, Import, ImportItem, ModuleDeclaration};

#[cfg(test)]
mod tests {
    use super::{
        BindingKind, Compiler, Context, Error, Import, ImportItem, ModuleDeclaration, Source, Stage,
        Token, TokenKind, shortcuts,
    };

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
    fn compiler_reports_parser_as_next_stage() {
        let compiler = Compiler::new();
        let error = compiler.compile(Source::new("test.astery", "fn main() {}"));

        assert_eq!(error.unwrap_err().stage(), Some(Stage::Parser));
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

    #[test]
    fn tokenizes_basic_function() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "fn main() { return }");
        let tokens = compiler.tokenize(&source).unwrap();

        assert_eq!(
            tokens.iter().map(Token::kind).collect::<Vec<_>>(),
            vec![
                &TokenKind::Fn,
                &TokenKind::Identifier("main".into()),
                &TokenKind::OpenParen,
                &TokenKind::CloseParen,
                &TokenKind::OpenBrace,
                &TokenKind::Return,
                &TokenKind::CloseBrace,
            ]
        );
    }

    #[test]
    fn tokenizes_literals_and_operators() {
        let compiler = Compiler::new();
        let source = Source::new(
            "main.as",
            "let value = 42 + 1.5; let ok = true && false; let c = 'x';",
        );
        let tokens = compiler.tokenize(&source).unwrap();
        let kinds = tokens.iter().map(Token::kind).collect::<Vec<_>>();

        assert!(kinds.contains(&&TokenKind::Integer("42".into())));
        assert!(kinds.contains(&&TokenKind::Float("1.5".into())));
        assert!(kinds.contains(&&TokenKind::And));
        assert!(kinds.contains(&&TokenKind::Character('x')));
    }

    #[test]
    fn tokenizes_labels_and_function_flags() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "@striped loop 'outer {} break 'outer");
        let tokens = compiler.tokenize(&source).unwrap();
        let kinds = tokens.iter().map(Token::kind).collect::<Vec<_>>();

        assert!(kinds.contains(&&TokenKind::At));
        assert!(kinds.contains(&&TokenKind::Identifier("striped".into())));
        assert!(kinds.contains(&&TokenKind::Label("outer".into())));
    }

    #[test]
    fn tokenizes_standard_library_names_as_identifiers() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "print eprint sizeof length format");
        let tokens = compiler.tokenize(&source).unwrap();

        assert_eq!(
            tokens.iter().map(Token::kind).collect::<Vec<_>>(),
            vec![
                &TokenKind::Identifier("print".into()),
                &TokenKind::Identifier("eprint".into()),
                &TokenKind::Identifier("sizeof".into()),
                &TokenKind::Identifier("length".into()),
                &TokenKind::Identifier("format".into()),
            ]
        );
    }

    #[test]
    fn reports_lexical_errors_with_position() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "fn main() { \"unterminated }");
        let error = compiler.tokenize(&source).unwrap_err();

        assert_eq!(error.stage(), Some(Stage::Lexer));
        assert!(error.to_string().contains("[1][13]"));
    }

    #[test]
    fn parses_direct_imports() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "use { mygame, standard, memory, engine }");
        let imports = compiler.parse_imports(&source).unwrap();

        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].module, None);
        assert_eq!(
            imports[0]
                .items
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<_>>(),
            vec!["mygame", "standard", "memory", "engine"]
        );
    }

    #[test]
    fn parses_nested_imports() {
        let compiler = Compiler::new();
        let source = Source::new(
            "main.as",
            "use math { function, variable { that } } use { memory }",
        );
        let imports = compiler.parse_imports(&source).unwrap();

        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].module.as_deref(), Some("math"));
        assert_eq!(imports[0].items[0].name, "function");
        assert_eq!(imports[0].items[1].name, "variable");
        assert_eq!(imports[0].items[1].items[0].name, "that");
        assert_eq!(imports[1].module, None);
        assert_eq!(imports[1].items[0].name, "memory");
    }

    #[test]
    fn parses_bare_import() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "use standard");
        let imports = compiler.parse_imports(&source).unwrap();

        assert_eq!(
            imports,
            vec![Import {
                module: Some("standard".into()),
                items: Vec::new(),
            }]
        );
    }

    #[test]
    fn parses_module_declaration() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "mod mygame");
        let module = compiler.parse_module(&source).unwrap();

        assert_eq!(
            module,
            ModuleDeclaration {
                name: "mygame".into()
            }
        );
    }

    #[test]
    fn parses_simple_let_binding() {
        let compiler = Compiler::new();
        let declarations = compiler
            .parse_bindings(&Source::new("main.as", "let name = value;"))
            .unwrap();

        assert_eq!(declarations.len(), 1);
        assert_eq!(declarations[0].kind, BindingKind::Let);
        assert_eq!(declarations[0].bindings.len(), 1);
        assert_eq!(declarations[0].bindings[0].name, "name");
        assert!(declarations[0].bindings[0].type_tokens.is_empty());
        assert!(declarations[0].value.is_some());
    }

    #[test]
    fn parses_typed_let_and_const_bindings() {
        let compiler = Compiler::new();
        let declarations = compiler
            .parse_bindings(&Source::new(
                "main.as",
                "let name string = value; const count i32 = 42;",
            ))
            .unwrap();

        assert_eq!(declarations.len(), 2);
        assert_eq!(declarations[0].kind, BindingKind::Let);
        assert_eq!(declarations[0].bindings[0].name, "name");
        assert_eq!(
            declarations[0].bindings[0].type_tokens,
            vec![TokenKind::Identifier("string".into())]
        );
        assert_eq!(declarations[1].kind, BindingKind::Const);
        assert_eq!(declarations[1].bindings[0].name, "count");
    }

    #[test]
    fn parses_multiple_let_bindings() {
        let compiler = Compiler::new();
        let declarations = compiler
            .parse_bindings(&Source::new("main.as", "let first, second, third = value;"))
            .unwrap();

        assert_eq!(declarations.len(), 1);
        assert_eq!(declarations[0].bindings.len(), 3);
        assert_eq!(declarations[0].bindings[0].name, "first");
        assert_eq!(declarations[0].bindings[1].name, "second");
        assert_eq!(declarations[0].bindings[2].name, "third");
    }

    #[test]
    fn parses_binding_block() {
        let compiler = Compiler::new();
        let declarations = compiler
            .parse_bindings(&Source::new(
                "main.as",
                "let { name string = value, other i32 = 42 };",
            ))
            .unwrap();

        assert_eq!(declarations.len(), 1);
        assert_eq!(declarations[0].kind, BindingKind::Let);
        assert_eq!(declarations[0].value, None);
        assert_eq!(declarations[0].bindings.len(), 2);
        assert_eq!(declarations[0].bindings[0].name, "name");
        assert_eq!(declarations[0].bindings[1].name, "other");
        assert!(declarations[0].bindings[0].value.is_some());
        assert!(declarations[0].bindings[1].value.is_some());
    }

    #[test]
    fn parses_underscore_binding() {
        let compiler = Compiler::new();
        let declarations = compiler
            .parse_bindings(&Source::new("main.as", "let _ = value;"))
            .unwrap();

        assert_eq!(declarations[0].bindings[0].name, "_");
    }

    #[test]
    fn reports_binding_parse_errors_with_position() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "let name;");
        let error = compiler.parse_bindings(&source).unwrap_err();

        assert_eq!(error.stage(), Some(Stage::Parser));
        assert!(error.to_string().contains("unexpected token"));
    }

    #[test]
    fn reports_import_parse_errors_with_position() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "use math { function, 42 }");
        let error = compiler.parse_imports(&source).unwrap_err();

        assert_eq!(error.stage(), Some(Stage::Parser));
        assert!(error.to_string().contains("expected identifier"));
    }

    #[test]
    fn import_result_is_stable() {
        let compiler = Compiler::new();
        let source = Source::new("main.as", "use math { function }");
        let imports = compiler.parse_imports(&source).unwrap();
        assert_eq!(
            imports,
            vec![Import {
                module: Some("math".into()),
                items: vec![ImportItem {
                    name: "function".into(),
                    items: Vec::new(),
                }],
            }]
        );
    }
}
