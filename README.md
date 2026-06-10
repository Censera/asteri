## Requirements

### To compile asteri*
- Rust 1.80+
- LLVM 20 (development headers)

### To run compiled programs
- `lld`, `ld.lld`, `cc`, `gcc`, `clang` Linux
- `lld-link`, `link.exe` Windows

### Installing LLVM 20

| OS | Command |
|----|---------|
| Ubuntu/Debian | `sudo apt install llvm-20-dev lld-20 clang-20` |
| macOS | `brew install llvm@20` |
| Windows | `choco install llvm --version=20.1.0` |
| NixOS | `nix-shell -p llvm_20 lld` or use `nix dev` for [flake.nix](/flake.nix) |
| Arch | `sudo pacman -S llvm20 lld` |
| Fedora | `sudo dnf install llvm20-devel lld20` |

> [!IMPORTANT]
> __Path to LLVM installation__: set `LLVM_SYS_201_PREFIX` if LLVM is installed in a non-standard location.
> __Link against libLLVM.so/dll instead of static `.a` files__: set `LLVM_LINK_SHARED=1` 
