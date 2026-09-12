{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    libxml2
  ] ++ (with llvmPackages_22; [
    llvm
    clang
    lld
  ]);

  shellHook = ''
    export LLVM_SYS_221_PREFIX="${pkgs.llvmPackages_22.llvm.dev}"
  '';
}
