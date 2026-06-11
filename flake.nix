{
    description = "asteri* lang";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
        flake-utils.url = "github:numtide/flake-utils";
    };
    outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
        flake-utils.lib.eachDefaultSystem (system:
        let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rust-bin.stable.latest.default;
            rustc = pkgs.rust-bin.stable.latest.default;
        };
        in {
            packages.default = rustPlatform.buildRustPackage {
                pname = "asteri";
                version = "0.3.0";
                src = ./.;
                nativeBuildInputs = with pkgs; [ llvm_20 ];
                buildInputs = with pkgs; [ llvm_20 lld_20 ];
                LLVM_SYS_201_PREFIX = "${pkgs.llvm_20}";
                cargoLock.lockFile = ./Cargo.lock;
        };

        devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
                (rust-bin.stable.latest.default.override {
                    extensions = [ "rust-src" "rust-analyzer" ];
                })
                llvm_20
                lld_20
                libxml2
                libffi
                zlib
                stdenv.cc
                libiconv
                tree
            ];

            LLVM_SYS_201_PREFIX = "${pkgs.llvm_20}";

            shellHook = ''
                clear
                echo -e '\n\t\x1b[1;45m ✱\x1b[0m asteri\n'
                tree --gitignore -C
            '';
        };
    });
}
