{
  inputs.nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.flake-parts.url = "github:hercules-ci/flake-parts";
  outputs =
    inputs@{
      self,
      nixpkgs,
      rust-overlay,
      flake-parts,
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        { pkgs, system, ... }:
        let
          name = "effective-rust";
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
          toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override {
            extensions = [ "rust-src" ];
          };
        in
        {
          devShells = {
            default = pkgs.mkShell {
              name = "effective-rust";
              buildInputs = with pkgs; [
                cargo-udeps
                cargo-nextest
                rust-analyzer
                sccache
              ];
              shellHook = ''
                export RUSTC_WRAPPER=${pkgs.sccache}/bin/sccache
                export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.stdenv.cc.cc.lib}/lib
              '';
            };
          };
        };
    };
}
