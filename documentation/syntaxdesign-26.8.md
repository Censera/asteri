# Syntax Re-design for 26

## Main features

```rs
// file: main.as
//! [type] is a placeholder for the language data types

// Module declaration
mod mygame

// Importing
use { mygame, standard, memory, engine }
use math { variable { that } }
use math { function, variable { that } }

// let
let name = value;
let name [type] = value;
let name, name, name = value;
let {
    name [type] = value,
    name [type] = value,
    name [type] = value
};
let _ = value;

// Constants
const name [type] = value;
const name, name, name = value;
const {
    name [type] = value,
    name [type] = value,
    name [type] = value
};

// fn
fn name() {}
fn name() { return }
fn [type] name() {
    return 0
}
name();

// Two functions can have the same name as long as they
// have different params or have different return type.
fn name() {}
fn [type] name() {}
fn [type] name(name type) {}

// Control Flow
if condition {}
if condition {} elif condition {} else {}
if condition {
    break
}
if condition {
    continue
} else {
    break
}

// Shortened
if condition then statement;
if condition then break

let something = value if conditon;
let somethingelse = value if conditon else value;


// loops
loop {}
loop 'name {
    loop {
        if condition then break
    }
    loop {
        if condition then break 'name
    }
}
while condition {}
match variable {
    value {},
    value {},
    value {}
}

// for loop
for i in items {}

// String Chains
// name name name // Spaced Chain "value value value\n"
// name..name..name.."\n" // Connected Chain "valuevaluevalue\n"
// Name can be any primitive type

print name name name;
print(name name name);

// Names, libraries, flags, and builtins
// Identifiers are ordinary names. Standard-library functions are identifiers too.
// An external standard library can provide functions without making them language builtins.
// @flags attach function settings or compiler-recognized behavior to functions.
// A builtin exists only when the language itself requires compiler-level support.

// Standard Library
let this string = "this";
let that string = "that";

print "Hello";
eprint "Error";
print "Hello" "world!"; // Hello world!\n
print "Hello".."world!".."\n"; // Hello world!\n
sizeof this; // 16
length this; // 4
format this "and" that "."; // this and that.
format("{} and {}.", this, that); // this and that.

// Shadowing is allowed

// read gets the input from stdin and outputs it
// as str then -> casts to a string.
let this string = read -> string;

// Enum
pub enum Name {}
pri enum Name {}
enum Name {
    pub name,
    pri name([type]),
    name(),
    pub name {
        name [type],
        name [type]
    },
    name {
        name [type],
        name [type]
    }
}

// Structures
pub struct Name {}
pri struct Name {}
struct Name {
    pub name [type],
    pri name [type],
}

// Get variable
print Name.name;

// Structure Implementation
into Name {
    pub fn [type] name() {}
    pri fn [type] name() {}
}

// Call function
print Name.name();

// User's types init using the type keyword
type Point i64;

type Node struct {};
into Node {
    fn Point new() {}
}
let node Node = Node.new();

embed C {
    printf("Hello\n");
}

// cast
// variable -> type

let a = true;
let b = a -> i8; // 1
let c = b -> char; // '1'
let d = c -> string; // "1"
let e = d -> char; // '1'
let f = e -> i8; // 1
let g = f -> bool; // true

// pointer
let name [type] = value;
let name ^[type] = &value;

// None pointer
let name ?^[type] = &value;
let name ?^[type] = None;

// Function flags for function's settings
@name

// from `name(arg)` to `name arg`
// only if it's one param
@striped
fn [type] name(name [type]) {}

@lossely
fn [type] name(name [type], ...) {}

@lossely
fn [type] name(name [type], ..., name [type], name [type], ...) {}

// Macros
macro this() { "this" }

macro square(x) {
    x * x
}

print this!();
print square!(4);
```

## Experimental

```rs
// Lambdas and Thunks
|t| t * 2
```

## Types

`[type]` is replaced with:

| Type                      | What it is                 |
| ------------------------- | -------------------------- |
| `i8`, `i16`, `i32`, `i64` | Signed integers            |
| `u8`, `u16`, `u32`, `u64` | Unsigned integers           |
| `f32`, `f64`              | Floating point             |
| `bool`                    | Boolean (`true` / `false`) |
| `char`                    | Character                  |
| `string`                  | Heap-allocated string      |
| `^T`                      | Non-null pointer to `T`    |
| `?^T`                     | Optional pointer to `T`    |

### Arrays

```rs
let name [type][length];
let name [type][] = [value, value, value];
let name [type][length] = [value, value, value];
let name [type][length, value];

print name[index];
print name[name[index]];
```

### Vectors

```rs
let name [type]<length>;
let name [type]<> = <value, value, value>;
let name [type]<length> = <value, value, value>;
let name [type]<length, value>;

print name<index>;
print name<name>;
print name<name<index>>;
```

### Tuples

```rs
let (name, name) [type];
let (name [type], name [type]);
let (name, name, name) = (value, value, value);
let (name, name, name) [type] = (value, value, value);
let (name [type], name [type]) = (value, value);
let (name [type], _) = (value, _);
let (_, name [type]) = (_, value);
let (_, name) [type] = (_, value);
let (name, _) [type] = (value, _);
let (_, _) = (value, value);

let (a, b) = (1, 2);
let (a, b) = (b, a);
```

## BitOP & LogicalOP etc.

```rs
// And
&&

// Or
||

// Xor
^^

// Not
!

// Bit And
:&

// Bit Or
:|

// Bit Xor
:^

// Bit Not
:<

// Shift Right
>>

// Shift Left
<<

// Inc
++

// Dec
--

// Operations
+, -, *, /
+=, -=, *=, /=
==, >=, <=, !=
```

## Outputs

### Success

```
Finished [bin] in X ms
```

```
Checked, and everything is OK.
```

### Warning

```
W [File][Line][Column] | Warning message
W [main.as][2][8] | Unused variable
```

## Error

```
E [File][Line][Column] | Error message
E [main.as][2][9] | Expected Semicolon `;`
```