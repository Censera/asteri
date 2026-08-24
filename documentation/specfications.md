<img src="/res/latest.svg" width="96" align="left">

__astery*__ is a domain-specific programming language designed for game development. It is compiled and statically typed. This project aims to provide an all-in-one environment for developers who like to work without separate engines and wrapper libraries.

 
 _This project is a work in progress, so many of the features displayed here may not exist yet and might never exist depending on what is right for the project._  
 
 _Also, this is the general-purpose core of astery*. Game development features (built-in game loop, graphics bindings, asset pipeline, physics types) are planned but not yet implemented. The language is fully usable for systems programming, scripting, and tooling in the future._

_See: __[Progress](prg.md)___

[![RepoRanker](https://reporanker.com/badge/Censera/astery)](https://reporanker.com/repos/Censera/astery)

## Syntax

### Hello, World

```rust
fn main() {
    print "Hello, World!";
}
```

`fn main() { .. }` declares the entry point. `main` takes no parameters and returns nothing by default. It is always the first code that runs.

### Variables

```rust
let x: i32 = 42;
let y = 100;
immut z: i32 = 7;
```

`let` declares a mutable variable
`immut` declares an immutable one
The type annotation is optional when an initializer is present

### Functions

```rust
fn: i32 add(a: i32, b: i32) {
    rt a + b;
}
```

Functions declare their return type after a colon. `rt` (or `return`) exits the function with a value.

```rust
fn say_hello() {
    print "Hello!";
}
```

A function with no return type returns nothing (void).

### Control Flow

```rust
if x > 0 {
    print "positive";
} else if x < 0 {
    print "negative";
} else {
    print "zero";
}
```

```rust
while counter < 10 {
    counter++;
}

```

```rust
loop {
    print 0;
    if done { break; }
}
```

### Match

```rust
match value {
    1: { print "one"; }
    2: { print "two"; }
    _: { print "other"; }
}
```

### Struct

```rust
struct Point {
    x: i32,
    y: i32,
    fn: i32 get_x() { rt x; }
}

let p = new Point { x: 10, y: 20 };
let px = p::get_x();
```

Structs group fields and methods. Methods are called with (::). Visibility is controlled with pub and pri.

### Pointers

```rust
let x: i32 = 10;
let p: ^i32 = &x;
p^ = 20;

let maybe: ?^i32 = &x;
let none_ptr: ?^i32 = None;
```

`^T` is a non-null pointer. `?^T` is an optional pointer that can be `None`. `&x` takes a reference. `p^` dereferences.

### Lambdas and Thunks

```rust
let double = |n| n * 2;
let result = double(21);

let compute = |a, b| {
    let t = a + b;
    t * 2;
};

// Thubks
lazy_val | expensive_work() |;
```

### Type Casts

```rust
let x: i64 = 42;
let y: i32 = x -> i32;     // i64 to i32
let f: f64 = x -> f64;     // int to float
let c: char = 65 -> char;  // int to char
```

### C introp

```rust
C {
    printf("hello from C\n");
}
```

### Types

|Type|What it is|
|----|---|
|`i8`, `i16`, `i32`, `i64`|Signed integers|
|`u8`, `u16`, `u32`, `u64`|Unsigned integers|
|`f32`, `f64`|Floating point|
|`bool`|Boolean (`true` / `false`)|
|`char`|Character|
|`str`|String slice|
|`String`|Heap-allocated string|
|`^T`|Non-null pointer to `T`|
|`?^T`|Optional pointer to `T`|

### Aliases

Most keywords have shorter aliases for quick scripting

|keyword| Its short form|
|----|---|
|`function`|`fn`|
|`return`|`rt`|
|`break`|`brk`|
|`continue`|`cnt`|
|`immutable`|`immut`|
|`structure`|`struct`|
|`enumeration`|`enum`|
|`private`|`pri`|
|`public`|`pub`|
|`int`|`i32`|
|`float`|`f32`|
|`double`|`f64`|
|`boolean`|`bool`|

For types, both the short and long forms are equivalent:

```rust
let x: int = 42;       // same as i32
let y: float = 3.14;   // same as f32
let z: double = 2.71;  // same as f64
let flag: bool = true; // same as boolean
```
