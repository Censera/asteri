# asteri*

A domain-specific programming language designed for game development. It is compiled and statically typed. This project aims to provide an all-in-one environment for developers who like to work without separate engines and wrapper libraries.

> [!IMPORTANT]
> The project is postponed  

## Hello World

```rust
fn main() {
    let hello = "Hello world!";
    print hello;
}
```

Output:

```text
Hello world!
```

## Installation

```bash
cargo install --git https://github.com/Censera/asteri
```

## Running Code

```bash
asteri run main.ast
```

## Language Overview

Functions:

```rust
fn greet(name: str) {
    print name;
}
```

Variables:

```rust
let score = 100;
```

Control Flow:

```rust
if score > 50 {
    print "Pass";
}
```

## Documentation

- Getting Started
- Progress & Roadmap

## License

[MIT](https://github.com/Censera/asteri/blob/entry/LICENSE)
