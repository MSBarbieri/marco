{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };
  outputs =
    { self
    , nixpkgs
    , utils
    , naersk
    , fenix
    , pre-commit-hooks
    ,
    }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        allowUnfree = true;
        overlays = [
          fenix.overlays.default
        ];
      };

      toolchain = with fenix.packages.${system}; combine [
        complete.cargo
        complete.rustc
        complete.rustfmt
        complete.rust-src
        complete.clippy
        # targets.x86_64-linux.latest.rust-std
        targets.wasm32-unknown-unknown.latest.rust-std
      ];

      naersk-lib = (naersk.lib.${system}.override {
        cargo = toolchain;
        rustc = toolchain;
      });
    in
    {
      defaultPackage = naersk-lib.buildPackage {
        src = ./.;

        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = with pkgs; [
          wasm-pack
          openssl
          protobuf
          pkg-config
        ];
        RUST_LOG = "trace";
      };
      checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
          settings = {
            clippy = {
              denyWarnings = true;
            };
          };
        };
      };

      devShell = nixpkgs.legacyPackages.${system}.mkShell {
        inherit (self.checks.${system}.pre-commit-check) shellHook;

        buildInputs = with pkgs; [
          toolchain
          cargo-watch
          nodejs-16_x
          pre-commit
          nixpkgs-fmt
          openssl
          protobuf
          wasm-pack
        ];

        RUST_LOG = "debug";

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
      };
    });
}
