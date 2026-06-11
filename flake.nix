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
                cargoLock.lockFile = ./Cargo.lock;
                doCheck = false;

                nativeBuildInputs = with pkgs; [
                llvmPackages_20.llvm
                llvmPackages_20.lld
                ];
                buildInputs = with pkgs; [ libffi ];

                preBuild = ''
                    export LLVM_SYS_201_PREFIX="${pkgs.llvmPackages_20.llvm.dev}"
                    export LIBCLANG_PATH="${pkgs.llvmPackages_20.llvm.lib}/lib"
                '';

        };

        devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
                (rust-bin.stable.latest.default.override {
                    extensions = [ "rust-src" "rust-analyzer" ];
                })
                llvmPackages_20.llvm
                llvmPackages_20.lld
                libxml2
                libffi
                zlib
                stdenv.cc
                libiconv
            ];

            LLVM_SYS_201_PREFIX = "${pkgs.llvmPackages_20.llvm.dev}";
            LIBCLANG_PATH = "${pkgs.llvmPackages_20.llvm.lib}/lib";

            shellHook = ''
                clear
                echo -e '\n\t\x1b[1;45m ✱\x1b[0m asteri\n'
        '';
        };
    });
}
