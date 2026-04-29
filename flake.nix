{
  description = "Safe Rust bindings for RTKLIB";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = (mkShell.override { stdenv = clangStdenv; }) {
          LIBCLANG_PATH = "${clangStdenv.cc.cc.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS =
            builtins.readFile "${clangStdenv.cc}/nix-support/libc-cflags"
            + " " + builtins.readFile "${clangStdenv.cc}/nix-support/cc-cflags";
          nativeBuildInputs = [
            git
            pkg-config
            rust-bin.nightly.latest.default
          ];
        };
      }
    );
}
