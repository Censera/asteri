# asteri

[![License](https://img.shields.io/github/license/Censera/asteri.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

A compiled, statically typed programming language for game development. The goal is a single self-contained toolchain: no separate engine, no wrapper libraries.

> Status: postponed. The compiler and test suite are checked in and buildable.

## Install

```r
cargo install --git https://github.com/Censera/asteri
```

**From source** (requires rustc 1.85+):

```r
cargo build --release
```

**Nix:**

```nix
nix build
```

## Usage

```
asteri run main.ast
```

## Language

**Hello World**

```rs
fn main() {
    let hello = "Hello world!";
    print hello;
}
```

**Functions**

```rs
fn greet(name: str) {
    print name;
}
```

**Variables**

```rs
let score = 100;
```

**Control Flow**

```rs
if score > 50 {
    print "Pass";
}
```

## Documentation

- [Getting Started](doc/)
- [Progress & Roadmap](mds/)

## License

[MIT](LICENSE)
