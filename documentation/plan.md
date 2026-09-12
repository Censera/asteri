# Plan

Astery grows from the language frontend toward native code generation.

## 1. Foundation

- Keep the v26 syntax document as the source of truth.
- Establish the Rust crate and compiler entry point.
- Establish explicit compiler errors.
- Keep source ownership and stage boundaries explicit.
- Keep the public compiler surface small.

## 2. Lexer

Implement the tokens required by the v26 syntax:

- Identifiers and keywords.
- Primitive literals.
- Strings and characters.
- Operators.
- Punctuation.
- Function flags and visibility markers.
- Pointer and cast syntax.

The lexer should operate directly on source bytes where practical and avoid unnecessary allocation.

## 3. Parser

Build the v26 AST for the main language surface:

- Imports.
- Bindings and constants.
- Functions.
- Blocks.
- Expressions.
- Control flow.
- Loops.
- Match expressions or statements.
- Arrays, vectors, and tuples.
- Enums.
- Structs and implementations.
- User-defined types.
- Casts and pointers.
- Embedded C blocks.

Experimental syntax is not required for the first complete frontend.

## 4. Semantic analysis

Add the rules that parsing alone cannot establish:

- Name resolution.
- Scope and shadowing.
- Visibility.
- Type checking.
- Function overload resolution.
- Return checking.
- Pointer rules.
- Cast validity.
- Break and continue targets.
- Struct and enum member resolution.
- Function flags.

Errors should identify the source construct that violated the rule.

## 5. LLVM interface

Use `an-inkwell` as the LLVM boundary.

The Astery backend should consume only the concrete LLVM operations it needs. It should not depend on Inkwell directly after the required `an-inkwell` surface exists and is exercised.

## 6. Code generation

Lower the semantically checked AST into LLVM IR in this order:

- Constants and primitive values.
- Local bindings.
- Arithmetic and logical operations.
- Comparisons.
- Casts.
- Function declarations and definitions.
- Calls and returns.
- Basic blocks.
- Conditional branches.
- Loops and loop control.
- Pointers, allocation, loads, and stores.
- Arrays, vectors, tuples, structs, and enums.

## 7. Standard library

Provide the documented core operations:

- `print`.
- `eprint`.
- `read`.
- `sizeof`.
- `length`.
- `format`.

These should remain ordinary, explicit compiler/runtime facilities.

## 8. Native output

Add target support and object or executable emission after real code generation is working.

## 9. Experimental features

Only after the main v26 language works, evaluate:

- Modules.
- Macros.
- Lambdas.
- Thunks.

The implementation should not introduce architecture for these features before they are required.
