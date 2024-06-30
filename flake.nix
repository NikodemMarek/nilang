{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [cargo2nix.overlays.default];
          };

          rustPkgs = pkgs.rustBuilder.makePackageSet {
            rustChannel = "nightly";
            extraRustComponents = ["rustfmt" "clippy"];
            packageFun = import ./Cargo.nix;
          };

          workspaceShell = let
            alias-run = pkgs.writeShellScriptBin "r" ''cargo run'';
            alias-dev = pkgs.writeShellScriptBin "d" ''${pkgs.cargo-watch}/bin/cargo-watch -C runner -x run -c'';
            alias-test = pkgs.writeShellScriptBin "t" ''${pkgs.cargo-watch}/bin/cargo-watch -C runner -x test -c'';
          in
            rustPkgs.workspaceShell
            {
              packages = [cargo2nix.packages."${system}".cargo2nix];
              buildInputs = [alias-run alias-dev alias-test];
              shellHook = ''
                printf "\e[33m
                  \e[1mr\e[0m\e[33m  -> run
                  \e[1md\e[0m\e[33m  -> dev
                  \e[1mt\e[0m\e[33m  -> tests
                \e[0m"
              '';
            };
        in rec {
          devShells = {
            default = workspaceShell;
          };

          packages = {
            nilang = rustPkgs.workspace.nilang {};
            default = packages.nilang;
          };
        }
      );
}
