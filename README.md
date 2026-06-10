<h1><img src="https://github.com/Censera/asteri/blob/entry/res/latest.png" width="64" align="center"> <span>asteri*</span></h1>
A compiled, statically typed DSL for game development.

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
| NixOS | `nix-shell -p llvm_20 lld` or use `nix dev` for [flake.nix](/flake.nix) |
| Arch | `sudo pacman -S llvm20 lld` |
| Fedora | `sudo dnf install llvm20-devel lld20` |

__For Windows__:
1. __Install Rust__: [`rustup`](https://rustup.rs)
2. __Install LLVM 20 from the official installer__:
   - Download [`LLVM-20.1.0-win64.exe`](https://github.com/llvm/llvm-project/releases/tag/llvmorg-20.1.0)
   - Run the installer, check "Add LLVM to the system PATH"
3. __Verify installation__:

```asm
llvm-config --version
```

5. __Build__:

```asm
cargo build
```

6. __If `llvm-config` is not found, set the environment variable manually__:

```asm
$env:LLVM_SYS_201_PREFIX = "C:\Program Files\LLVM"
```

> [!TIP]
> __Path to LLVM installation__: set `LLVM_SYS_201_PREFIX` if LLVM is installed in a non-standard location.  
> __Link against `libLLVM.so/dll` instead of static `.a` files__: set `LLVM_LINK_SHARED=1`

## Quick Start

### Install asteri*

```asm
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

```asm
./asteri run hello.ast
```

3. Output:

```asm
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
