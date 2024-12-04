{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
  in {
    devShells.${system}.default = let
      pkgs = import nixpkgs { inherit system; };
    in pkgs.mkShell {
      name = "advent-of-code-2025";
      buildInputs = with pkgs; [
        cargo
        aoc-cli
        rustc
        rust-analyzer
        rustfmt
        rustPackages.clippy
      ];
        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
    };
  };
}
