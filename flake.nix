{
    description = "asteri* dev env";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs }:
        let
            system = "x86_64-linux";
            pkgs = nixpkgs.legacyPackages.${system};
        in
        {
            devShells.${system}.default = pkgs.mkShell {
            buildInputs = with pkgs; [
                llvmPackages_20.llvm
                libxml2
                libffi
                zlib
                stdenv.cc

                rustc
                cargo
                libiconv
            ];

            shellHook = ''
                export LLVM_SYS_201_PREFIX="${pkgs.llvmPackages_20.llvm.dev}"

                clear
                echo -e '\t* asteri'
                ${pkgs.tree}/bin/tree --gitignore -C

                exec fish
            '';
            };
        };
}
