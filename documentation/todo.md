# TODO

## Foundation

- [x] Define the v26 syntax source of truth.
- [x] Establish the compiler library and executable entry point.
- [x] Establish explicit compiler errors.
- [x] Keep compiler stage boundaries explicit.

## Lexer

- [x] Tokenize module declarations, identifiers, and keywords.
- [x] Tokenize primitive literals.
- [x] Tokenize strings and characters.
- [x] Tokenize operators and punctuation.
- [x] Tokenize visibility and function flags.
- [x] Tokenize pointer and cast syntax.
- [x] Tokenize ranges and inclusive ranges.
- [x] Tokenize collection delimiters and literals.
- [x] Tokenize macro syntax.
- [x] Tokenize embedded C blocks.
- [x] Tokenize union type syntax.

## Parser

- [x] Parse module declarations.
- [x] Parse imports.
- [x] Parse `let` and `const` bindings.
- [x] Parse functions and overload declarations.
- [x] Parse function flags.
- [x] Parse blocks and expressions.
- [x] Parse calls and member access.
- [x] Parse `if`, `elif`, `else`, `loop`, `while`, `for`, and `match`.
- [x] Parse shortened conditional statements and expressions.
- [x] Parse `break` and `continue` with labels.
- [x] Parse range and inclusive-range expressions.
- [ ] Parse string chains.
- [x] Parse arrays, vectors, and tuples.
- [ ] Parse enums and enum variants.
- [ ] Parse structs and fields.
- [ ] Parse `into` implementations.
- [ ] Parse user-defined types.
- [ ] Parse casts and pointers.
- [ ] Parse embedded C blocks.
- [ ] Parse macros and macro calls.

## Semantic analysis

- [ ] Resolve modules and imports.
- [ ] Resolve names.
- [ ] Resolve scopes and shadowing.
- [ ] Check visibility.
- [ ] Check primitive and user-defined types.
- [ ] Check arrays, vectors, tuples, and their indexing/access rules.
- [ ] Resolve overloaded functions.
- [ ] Resolve `into` implementations and methods.
- [ ] Check returns.
- [ ] Check casts.
- [ ] Check pointer and optional-pointer rules.
- [ ] Check control-flow targets and labels.
- [ ] Resolve struct and enum members.
- [ ] Validate function flags.
- [ ] Validate string-chain operands.
- [ ] Validate embedded C boundaries.

## Backend

- [ ] Define the Astery to `an-inkwell` boundary.
- [ ] Lower constants and primitive values.
- [ ] Lower local bindings.
- [ ] Lower arithmetic and logical operations.
- [ ] Lower comparisons.
- [ ] Lower ranges.
- [ ] Lower casts.
- [ ] Lower functions.
- [ ] Lower calls and method calls.
- [ ] Lower returns.
- [ ] Lower basic blocks.
- [ ] Lower branches and loops.
- [ ] Lower pointers, allocation, loads, and stores.
- [ ] Lower arrays, vectors, and tuples.
- [ ] Lower structs and enums.
- [ ] Lower user-defined types.
- [ ] Lower embedded C.

## Standard library

- [ ] Implement `print`.
- [ ] Implement `eprint`.
- [ ] Implement `read`.
- [ ] Implement `sizeof`.
- [ ] Implement `length`.
- [ ] Implement `format`.
- [ ] Implement `alloc`.
- [ ] Implement `free`.
- [ ] Implement `parse`.
- [ ] Implement array/vector push, pop, and length operations.

## Native output

- [ ] Add target initialization through `an-inkwell`.
- [ ] Add target-machine support when required.
- [ ] Emit native object code or executables.

## Experimental

- [ ] Evaluate macros as a language feature boundary.
- [ ] Evaluate lambdas.
- [ ] Evaluate thunks.
- [ ] Evaluate additional module features.
