{ task-lib }:
with task-lib;
let 
    root = task-lib.snips.git-find-root;
in
rec { 

    build = mkTask "build" { script = /*bash*/ ''
        cargo build --release
    ''; };

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
                "mkdir -p tests/ ; echo 'Recording example {}' ; ./target/release/ssl examples/{} --ast > tests/{}.ast"
            '';
        depends = [ build ];
    };

    check-examples = mkTask "check-examples" {
        script = /*bash*/ ''
            ls ./examples | xargs -I{} bash -c \
                "echo 'Checking example {}' ; ./target/release/ssl examples/{} --ast | diff - tests/{}.ast || exit 1" \
                || { echo "Example AST has changed." && exit 1 ; }
            '';
        depends = [ build ];
    };

    demo-build = mkTask "demo-build" {
        script = /*bash*/ ''
            wasm-pack build --target web "`${root}`/demo"
        '';
    };

    demo-build-release = mkTask "demo-build" {
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
            echo "Stupid Shader Language" | cargo run -- /dev/stdin -W 100 -H 100 -o $root/demo/favicon.png
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
            cp -v "$root/demo/index.css" "$root/demo/public"
            cp -v "$root/demo/index.js" "$root/demo/public"
        '';
        depends = [
            demo-build
            demo-generate-favicon
        ];
    };

    gen-scripts = task-lib.gen.gen-scripts "gen-scripts";
}
