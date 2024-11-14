{ task-lib }:
with task-lib;
let 
    root = snips.git-find-root;

    manifest-path = x: ''--manifest-path "`${root}`/${x}/Cargo.toml"'';

    cargo-fmt = x: /*bash*/ ''
        cargo fmt --check ${manifest-path x} \
            || { echo -e "\nPlease format your files in '${x}'.";  exit 1; }
    '';

    cargo-clippy = x: /*bash*/ ''
        cargo clippy ${manifest-path x} -- --deny warnings 2> /dev/null \
            || { echo -e "\nClippy is angry in '${x}'."; exit 1; }
    '';

    cargo-build = dir: /*bash*/ ''
        cargo build --release ${manifest-path dir}
    '';

in
rec { 

    build = mkTask "build" { script = cargo-build "./."; };

    check-fmt = mkTask "check-fmt" { script = ''
            ${ cargo-fmt "./." }
            ${ cargo-fmt "./demo/" }
        '';
    };

    check-clippy = mkTask "check-clippy" { script = ''
            ${ cargo-clippy "./." }
            ${ cargo-clippy "./demo/" }
       '';
    };

    run-examples = mkTask "run-examples" {
        script = /*bash*/ ''
            ls ./examples | xargs -I{} bash -c \
                "mkdir -p art/ ; echo 'Running example {}'; ./target/release/ssl examples/{} --output art/{}.png"
            '';
        depends = [ build ];
    };

    record-examples = mkTask "record-examples" {
        script = /*bash*/ ''
            ls ./examples | xargs -I{} bash -c \
                "mkdir -p tests/ ; echo 'Recording example {}' ; ./target/release/ssl examples/{} --ast --dry-run > tests/{}.ast"
            '';
        depends = [ build ];
    };

    check-examples = mkTask "check-examples" {
        script = /*bash*/ ''
            ls ./examples | xargs -I{} bash -c \
                "echo 'Checking example {}' ; ./target/release/ssl examples/{} --ast --dry-run | diff - tests/{}.ast || exit 1" \
                || { echo "Example AST has changed." ; exit 1 ; }
            '';
        depends = [ build ];
    };

    demo-build = mkTask "demo-build" {
        script = /*bash*/ ''
            wasm-pack build --target web "`${root}`/demo"
        '';
    };

    demo-build-release = mkTask "demo-build-release" {
        script = /*bash*/ ''
            wasm-pack build --release --target web "`${root}`/demo"
        '';
    };

    demo-serve = mkTask "demo-serve" {
        script = /*bash*/ ''
            cd "`${root}`/demo" && python3 -m http.server
        '';
        depends = [ demo-build ];
    };

    demo-watch = mkTask "demo-watch" {
        script = /*bash*/ ''
            root="`${root}`"
            inotifywait -r -m --exclude "(pkg)|(target)|(public)" -e modify "$root/demo" | 
                while read file_path file_event file_name; do 
                    echo -e "\nFile changed: $file_path/$file_name"
                    wasm-pack build --target web "$root/demo"
                done
        '';
        depends = [ demo-build ];
    };

    demo-generate-favicon = mkTask "demo-generate-favicon" {
        script = /*bash*/ ''
            root="`${root}`"
            echo "Stupid Shader Language" | cargo run -- /dev/stdin -W 16 -H 16 -o $root/demo/favicon.png
            mv -v $root/demo/favicon.png $root/demo/favicon.ico
        '';
    };

    demo-package = mkTask "demo-package" {
        script = /*bash*/ ''
            root="`${root}`"
            mkdir -p "$root/demo/public"
            cp -rv "$root/demo/pkg" "$root/demo/public"
            cp -v "$root/demo/favicon.ico" "$root/demo/public"
            cp -v "$root/demo/index.html" "$root/demo/public"
            cp -v "$root/demo/styles.css" "$root/demo/public"
            cp -v "$root/demo/index.js" "$root/demo/public"
        '';
        depends = [
            demo-build
        ];
    };

    gen-scripts = gen.gen-scripts "gen-scripts";

    gen-random = mkTask "gen-random" {
        script = /*bash*/ ''
            # Get argument if provided
            len=$1
            if [ -z "$len" ]; then
                len=50
            fi

            input="$(tr -dc A-Za-z0-9 </dev/urandom | head -c $len)"
            echo "Running input: $input"
            echo "$input" | cargo run -r -- /dev/stdin
        '';
    };

    pre-push = mkSeq "pre-push" [
        check-fmt
        check-clippy
        check-examples
        gen-scripts
        demo-generate-favicon
        (task-lib.gen.check-no-uncommited "Please commit your changes before pushing.")
    ];

}
