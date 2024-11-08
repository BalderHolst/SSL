{ task-lib }:
with task-lib;
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

    gen-scripts = task-lib.gen.gen-scripts "gen-scripts";
}
