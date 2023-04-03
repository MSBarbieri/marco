{
  inputs = {
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

    in
    {
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

      devShells.default = pkgs.mkShell {
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
