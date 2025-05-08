{
  description = "Nix flake for hash-tables-without-assumptions-private";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable."1.86.0".default;
        pythonEnv = pkgs.python311.withPackages (ps: with ps; [
          matplotlib pandas seaborn
        ]);
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            rust-analyzer
            cargo-watch
            cargo-nextest
            cargo-audit
            cargo-deny
            cargo-udeps
            cargo-sort
            cargo-expand
            cargo-bloat
            gnuplot
            just
            typos
            pythonEnv
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          shellHook = ''
            export LANG=en_AU.UTF-8
            export LANGUAGE=en_AU.UTF-8

            echo "Rust + Python dev shell ready."
            echo "Use 'just' to see available tasks."
          '';
        };
      });
}
