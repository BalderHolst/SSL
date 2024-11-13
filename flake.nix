rec {
    description = "Full demo of task-gen library";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
        task-gen.url = "github:BalderHolst/task-gen.nix";
    };

    outputs = { nixpkgs, flake-utils, task-gen, ... }:
    flake-utils.lib.eachDefaultSystem (system:
        let
            task-lib = task-gen.lib."${system}";
            tasks = import ./tasks.nix { inherit task-lib; };
            pkgs = import nixpkgs { inherit system; };
        in
        {

            apps = {
                gen-scripts = with task-lib; mkGenScriptsApp {
                    "scripts/build.sh" = mkScript tasks.build;
                    "scripts/package.sh" = mkScript tasks.demo-package;
                    ".hooks/pre-commit" = mkScript tasks.check-fmt;
                    ".hooks/pre-push" = mkScript tasks.pre-push;
                };
            };

            devShell = with pkgs; mkShell {
                buildInputs = [
                    wasm-pack     # Rust wasm packager
                    rustc-wasm32  # Rust wasm target
                    cargo         # Rust package manager
                    clippy        # Rust linter
                    lld           # Wasm linker
                    python3       # For web development server
                    inotify-tools # For live reloading
                ] ++ (task-lib.mkScripts tasks);

                shellHook = ''
                    echo -e "${description}\n"
                    git config --local core.hooksPath .hooks
                '' + task-lib.mkShellHook tasks;
            };

    });
}
