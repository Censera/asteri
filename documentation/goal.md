# Goal

Astery is a statically typed programming language for game development.

The v26 goal is a small compiled language with direct control over its execution model, a clear syntax, and an implementation that remains understandable from the language surface down to generated code.

## Language

Astery should provide:

- Static types.
- Functions and function overloading.
- Local bindings and constants.
- Structured control flow.
- Arrays, vectors, tuples, structs, and enums.
- Pointers and optional pointers.
- Explicit casts.
- Visibility control.
- A small standard library.
- C embedding where required.

The syntax design is defined by [`syntaxdesign-26.8.md`](syntaxdesign-26.8.md).

## Compiler

The compiler should turn valid Astery source into native code through LLVM.

The implementation should grow in the same order as the language requires it:

```text
source
  -> lexer
  -> parser
  -> AST
  -> semantic analysis
  -> LLVM IR
  -> native output
```

Each stage owns one responsibility. Later stages must not compensate for missing invariants in earlier stages.

## Backend

Astery should consume [`an-inkwell`](https://github.com/Censera/an-inkwell) for LLVM interaction instead of exposing LLVM's full API directly to the compiler.

The backend should use only the operations the language actually needs.

## Standard library

The standard library should start small. `print`, `eprint`, `read`, `sizeof`, `length`, and `format` are language-facing operations documented by v26.

Library functionality should be added when the language or runtime has a concrete need for it, not as a general-purpose framework.

## Scope

The first complete v26 implementation does not require every experimental language feature.

The experimental sections of the syntax document are separate from the main language surface and do not define the minimum implementation target.
