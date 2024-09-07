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
            alias-test = pkgs.writeShellScriptBin "t" ''${pkgs.cargo-watch}/bin/cargo-watch -C generator -x test -c'';

            alias-assemble = pkgs.writeShellScriptBin "ae" ''as test.asm -o test.o'';
            alias-link = pkgs.writeShellScriptBin "lk" ''ld test.o -o test'';
            alias-execute = pkgs.writeShellScriptBin "ee" ''./test'';
            alias-assemble-link-execute = pkgs.writeShellScriptBin "alr" ''ae && lk && ee'';
            alias-run-assemble-link-execute = pkgs.writeShellScriptBin "ralr" ''r && ae && lk && ee ; echo $?'';
          in
            rustPkgs.workspaceShell
            {
              packages = [cargo2nix.packages."${system}".cargo2nix];
              buildInputs = [alias-run alias-dev alias-test alias-assemble alias-link alias-execute alias-assemble-link-execute alias-run-assemble-link-execute];
              shellHook = ''
                printf "\e[33m
                  \e[1mr\e[0m\e[33m  -> run
                  \e[1md\e[0m\e[33m  -> dev
                  \e[1mt\e[0m\e[33m  -> tests

                  \e[1mae\e[0m\e[33m  -> assemble
                  \e[1mlk\e[0m\e[33m  -> link
                  \e[1mee\e[0m\e[33m  -> execute

                  \e[1malr\e[0m\e[33m  -> assemble, link, execute
                  \e[1mralr\e[0m\e[33m -> run, assemble, link, execute
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
