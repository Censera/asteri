# TODO

## Foundation

- [x] Define the v26 syntax source of truth.
- [ ] Establish the compiler library and executable entry point.
- [ ] Establish explicit compiler errors.
- [ ] Keep compiler stage boundaries explicit.

## Lexer

- [ ] Tokenize identifiers and keywords.
- [ ] Tokenize primitive literals.
- [ ] Tokenize strings and characters.
- [ ] Tokenize operators and punctuation.
- [ ] Tokenize visibility and function flags.
- [ ] Tokenize pointer and cast syntax.

## Parser

- [ ] Parse imports.
- [ ] Parse `let` and `const` bindings.
- [ ] Parse functions and overloads.
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

## Native output

- [ ] Add target initialization through `an-inkwell`.
- [ ] Add target-machine support when required.
- [ ] Emit native object code or executables.

## Experimental

- [ ] Evaluate modules.
- [ ] Evaluate macros.
- [ ] Evaluate lambdas.
- [ ] Evaluate thunks.
