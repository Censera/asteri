<h1><img src="res/latest.svg" width="64" align="center"> <span>asteri*</span></h1>

A compiled, statically typed DSL for game development. (WIP)

## Overview

- __[Quick Start](#quick-start)__
- __[Usage](#usage)__
- __[Requirements](#requirements)__

## Quick Start

### Install asteri*

See: [Requirements](#requirements)

```bash
cargo install --git https://github.com/Censera/asteri
```

### Hello, World!

1. Create `hello.ast`:

```rust
fn main() {
    print "Hello, world!";
}
```

2. Compile and run:

```bash
./asteri run hello.ast
```

3. Output:

```bash
Hello, world!
```

## Usage

__Commands__

| Command | Description |
|----|----|
| `asteri run <file>`	| Compile and execute |
| `asteri build <file>`	| Compile to native binary |
| `asteri check <file>`	| Type-check only |

__Options__

Flag |	Description |
|----|----|
| `-o, --output <path>`	| Output file or directory |
| `-n --name <name>`	| Module/binary name |
| `-k, --keep`	| Keep intermediate files |

__Help__

```r
            asteri* v0.3.0

    Usage:
        asteri <command> [options] <file>

    Commands:
        run         Compile and run the program
        build       Build the binary
        check       Typecheck
        dev         For fast iteration
        version     Show version
        help        Display this message

    Options:
        -o --output <path>      To specify an output path
        -n --name <name>        For naming the output files
        -k --keep               Keep .ll and .o
```

## Requirements

### To compile asteri*

> [!IMPORTANT]
> - Rust 1.80+  
> - LLVM __20.x__ Must match exactly. 19.x or 21.x will not work.

### To run compiled programs

- `lld`, `ld.lld`, `cc`, `gcc`, `clang` Linux or macOS
- `lld-link`, `link.exe` Windows

### Installing LLVM 20

| OS | Command |
|----|---------|
| Ubuntu/Debian | `sudo apt install llvm-20-dev lld-20 clang-20` |
| macOS | `brew install llvm@20` |
| Arch | `sudo pacman -S llvm20 lld` |
| Fedora | `sudo dnf install llvm20-devel lld20` |

#### For Nix:

```nix
nix flake update github:Censera/asteri
nix run github:Censera/asteri -- run hello.ast
```

```nix
nix run github:Censera/asteri --refresh -- run hello.ast
```

__If flake fails__:

Update your flake inputs

```nix
nix flake update --flake github:Censera/asteri
```

Or use the dev shell instead

```nix
nix develop github:Censera/asteri
cargo run -- run hello.ast
```

#### For Windows:

1. __Install Rust__: [`rustup`](https://rustup.rs)
2. __Install LLVM 20 from the official installer__:
    - Download [`LLVM-20.1.0-win64.exe`](https://github.com/llvm/llvm-project/releases/tag/llvmorg-20.1.0)
    - Run the installer, check "Add LLVM to the system PATH"
3. __Verify installation__:

```bash
llvm-config --version
```

5. __Build__:

```bash
cargo build
```

6. __If `llvm-config` is not found, set the environment variable manually__:

```bash
$env:LLVM_SYS_201_PREFIX = "C:\Program Files\LLVM"
```

> [!TIP]
> __Path to LLVM installation__: set `LLVM_SYS_201_PREFIX` if LLVM is installed in a non-standard location.  
> __Link against `libLLVM.so/dll` instead of static `.a` files__: set `LLVM_LINK_SHARED=1`
