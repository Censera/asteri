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
- [ ] Tokenize collection delimiters and literals.
- [ ] Tokenize macro syntax.
- [ ] Tokenize embedded C blocks.

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
- [ ] Parse arrays, vectors, and tuples.
- [ ] Parse enums and enum variants.
- [ ] Parse structs and fields.
- [ ] Parse `into` implementations.
- [ ] Parse user-defined types.
- [ ] Parse casts and pointers.
- [ ] Parse embedded C blocks.
- [ ] Parse macros and macro calls.

## Semantic analysis

### Names and modules

- [ ] Build the module tree from `mod` declarations.
- [ ] Resolve imports and imported members.
- [ ] Resolve every identifier to its declaration.
- [ ] Detect duplicate declarations in the same scope.
- [ ] Define and enforce scope boundaries.
- [ ] Define and enforce shadowing rules.
- [ ] Check `pub` and `pri` visibility across modules and implementations.
- [ ] Report unresolved names with their source location and scope context.

### Types

- [ ] Define the semantic type model for all primitive types.
- [ ] Define the semantic representation of `None`.
- [ ] Define pointer and optional-pointer types.
- [ ] Define arrays, vectors, and tuples as distinct semantic types.
- [ ] Register and resolve user-defined types.
- [ ] Register and resolve enums and their variants.
- [ ] Register and resolve structs and their fields.
- [ ] Check explicit type annotations against inferred values.
- [ ] Define which values can be inferred without an annotation.
- [ ] Check literal types and valid literal conversions.
- [ ] Check operator operand types and result types.
- [ ] Check comparison and logical operator rules.
- [ ] Check indexing and member-access types.
- [ ] Check range endpoint types and inclusive-range rules.
- [ ] Check assignment and binding type compatibility.

### Functions and calls

- [ ] Build function signatures from names, parameters, and return types.
- [ ] Register overloaded functions without losing distinct signatures.
- [ ] Resolve a call against the available overloads.
- [ ] Define overload resolution and ambiguity rules.
- [ ] Check argument count and argument types.
- [ ] Check return expressions against the declared return type.
- [ ] Check functions that do not return a value.
- [ ] Resolve member calls through `into` implementations.
- [ ] Check method receiver/member compatibility.
- [ ] Validate variadic arguments for `@lossely` functions.
- [ ] Validate the restricted call form introduced by `@striped`.
- [ ] Validate supported and reserved function flags.

### Expressions and values

- [ ] Type-check every expression recursively.
- [ ] Define expression result types for unary and binary operators.
- [ ] Define how `None` participates in expressions and bindings.
- [ ] Define evaluation rules for shortened conditional expressions.
- [ ] Check `if` and `elif` conditions are valid conditions.
- [ ] Check `while` conditions are valid conditions.
- [ ] Check `for` iteration values and loop variables.
- [ ] Check `match` values and pattern compatibility.
- [ ] Check string-chain operands and conversion rules.
- [ ] Define member access for values, structs, enums, and supported types.

### Casts and pointers

- [ ] Define the complete set of valid casts.
- [ ] Reject invalid casts with the source and target types.
- [ ] Check pointer construction and referenced value types.
- [ ] Check optional-pointer construction with `None`.
- [ ] Check pointer dereference rules when added to the expression model.
- [ ] Define pointer equality/comparison rules if supported.
- [ ] Preserve pointer nullability through expressions and calls.

### Control flow

- [ ] Validate `break` targets.
- [ ] Validate `continue` targets.
- [ ] Resolve named loop labels.
- [ ] Reject references to labels outside their valid loop scope.
- [ ] Reject `break` and `continue` outside loops.
- [ ] Define control-flow reachability for returns and loop exits.
- [ ] Check functions return correctly on every required path.

### Collections and aggregates

- [ ] Check array element types and declared sizes.
- [ ] Check vector element types.
- [ ] Check tuple element count and positional types.
- [ ] Check destructuring bindings against aggregate shapes.
- [ ] Check aggregate literals against their expected types.

### User-defined types and implementations

- [ ] Check enum variant declarations and payload types.
- [ ] Check struct field declarations and duplicate fields.
- [ ] Check `into` implementations target valid types.
- [ ] Check implementation member visibility and signatures.
- [ ] Reject conflicting or duplicate implementation members.
- [ ] Resolve associated functions and members on user-defined types.

### Compiler-recognized behavior

- [ ] Validate compiler-recognized function flags before lowering.
- [ ] Define which flags affect syntax, typing, code generation, or diagnostics.
- [ ] Define builtin operations that require compiler support separately from standard-library identifiers.
- [ ] Validate `embed C` boundaries and required compile-time constraints.
- [ ] Validate macro inputs and expansion boundaries when macros are enabled.

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
