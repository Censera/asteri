# TODO

## Foundation

- [x] Define the v26 syntax source of truth.
- [x] Establish the compiler library and executable entry point.
- [x] Establish explicit compiler errors.
- [x] Keep compiler stage boundaries explicit.

## Lexer

- [x] Tokenize identifiers and keywords.
- [x] Tokenize primitive literals.
- [x] Tokenize strings and characters.
- [x] Tokenize operators and punctuation.
- [x] Tokenize visibility and function flags.
- [x] Tokenize pointer and cast syntax.

## Parser

- [x] Parse imports.
- [x] Parse `let` and `const` bindings.
- [x] Parse functions and overloads.
- [ ] Parse blocks and expressions.
- [ ] Parse `if`, `elif`, `else`, `loop`, `while`, `for`, and `match`.
- [ ] Parse `break` and `continue` with labels.
- [ ] Parse string chains.
- [ ] Parse arrays, vectors, and tuples.
- [ ] Parse enums.
- [ ] Parse structs and `into` implementations.
- [ ] Parse user-defined types.
- [ ] Parse casts and pointers.
- [ ] Parse embedded C blocks.

## Semantic analysis

- [ ] Resolve names.
- [ ] Resolve scopes and shadowing.
- [ ] Check visibility.
- [ ] Check types.
- [ ] Resolve overloaded functions.
- [ ] Check returns.
- [ ] Check casts.
- [ ] Check pointer rules.
- [ ] Check control-flow targets.
- [ ] Resolve struct and enum members.
- [ ] Validate function flags.

## Backend

- [ ] Define the Astery to `an-inkwell` boundary.
- [ ] Lower constants and primitive values.
- [ ] Lower local bindings.
- [ ] Lower arithmetic and logical operations.
- [ ] Lower comparisons.
- [ ] Lower casts.
- [ ] Lower functions.
- [ ] Lower calls.
- [ ] Lower returns.
- [ ] Lower basic blocks.
- [ ] Lower branches and loops.
- [ ] Lower pointers, allocation, loads, and stores.
- [ ] Lower arrays, vectors, and tuples.
- [ ] Lower structs and enums.

## Standard library

- [ ] Implement `print`.
- [ ] Implement `eprint`.
- [ ] Implement `read`.
- [ ] Implement `sizeof`.
- [ ] Implement `length`.
- [ ] Implement `format`.
- [ ] Implement `alloc`.
- [ ] Implement `free`.
- [ ] Implement `parse`
- [ ] Implement `array`/`vector` push/pop/len.


## Native output

- [ ] Add target initialization through `an-inkwell`.
- [ ] Add target-machine support when required.
- [ ] Emit native object code or executables.

## Experimental

- [ ] Evaluate modules.
- [ ] Evaluate macros.
- [ ] Evaluate lambdas.
- [ ] Evaluate thunks.
