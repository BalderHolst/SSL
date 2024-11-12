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
                    "Makefile" = mkMakefile tasks;
                };
            };

            devShell = with pkgs; mkShell {
                buildInputs = [
                    wasm-pack     # Rust wasm packager
                    rustc-wasm32  # Rust wasm target
                    lld           # Wasm linker
                    python3       # For web development server
                    inotify-tools # For live reloading
                ] ++ (task-lib.mkScripts tasks);

                shellHook = ''
                    echo -e "${description}\n"
                '' + task-lib.mkShellHook tasks;
            };

    });
}
