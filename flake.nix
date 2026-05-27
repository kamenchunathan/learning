{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, naersk }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    in
    {
      overlays.default = final: prev: {
        rustToolchain = prev.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      };
    }
    //
    flake-utils.lib.eachSystem supportedSystems (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            self.overlays.default
          ];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = pkgs.rustToolchain;
          rustc = pkgs.rustToolchain;
        };

        gungraun-runner = naersk'.buildPackage {
          name = "gungraun-runner";
          src = pkgs.fetchFromGitHub {
            owner = "gungraun";
            repo = "gungraun";
            rev = "v0.19.0";
            hash = "sha256-K/poiIlqZQO/FK7zZzSTGGZHvsPUE0qPwxVHJylTSU0=";
          };
          # The source is a workspace; tell cargo which crate to build.
          cargoBuildOptions = old: old ++ [ "--package" "gungraun-runner" ];
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };
      in
      {
        packages.gungraun-runner = gungraun-runner;

        devShells.default = pkgs.mkShell.override
          { stdenv = pkgs.clangStdenv; }
          {
            packages = with pkgs; [
              # rust toolchain
              rustToolchain
              openssl
              pkg-config
              cargo-watch
              rust-analyzer

              # command runner
              just

              valgrind
              gungraun-runner
            ];

            env = {
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
      });
}






