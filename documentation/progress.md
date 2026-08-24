## Progress

### Compiles to native binary
- `fn main() { ... }`: entry point
- `fn: i32 main() { rt 42; }`: function with return value
- `let x: i32 = 42;`/`let x = 42;`: variable declaration with type inference
- `immut x: i32 = 7;`: immutable variables
- `print 42;`/`print "hello";`: integer and string output
- Arithmetic: `+`, `-`, `*`, `/`
- Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Bitwise: `BitOr`, `BitAnd`, `BitXor`, `BitNot`
- Type casts: `x -> i64`
- `None` literal for optional pointers

### Parses and type-checks (not yet compiled)
- `if`/`else`: control flow
- `while`, `loop`, `break`, `continue`
- `match`: pattern matching
- Struct definitions with methods
- Lambdas and thunks
- Pointers, optional pointers, dereference
- Format strings `f"..."`
- C blocks `C { ... }`

### Planned
- Code generation for control flow
- Code generation for function calls
- Code generation for structs
- Arrays, modules, standard library
- Game development features
