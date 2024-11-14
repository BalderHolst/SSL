use clap::Parser;
use std::{fs, process::exit};

mod ast;
mod cli;
mod evaluator;
mod lexer;
mod parser;
mod renderer;
mod text;

fn main() {
    let opts = cli::Cli::parse();

    let source = match fs::read_to_string(&opts.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", opts.input.display(), e);
            exit(1);
        }
    };

    let lexer = lexer::Lexer::new(source);
    let source = lexer.source();
    let tokens: Vec<_> = lexer.collect();

    let mut parser = parser::Parser::new(tokens, source.clone());
    let expr = parser.parse_expr();

    if opts.print_expr {
        println!("{}", expr);
    }

    if opts.print_ast {
        expr.print_ast(source);
    }

    if opts.dry_run {
        return;
    }

    const PARTS: u32 = 10;
    let f = match opts.verbose {
        true => |part| println!("Rendering {}% ...", (part + 1) * 100 / PARTS),
        false => |_| {},
    };
    let image = renderer::render_in_parts(&expr, opts.width, opts.height, PARTS, f);

    let out_file = &opts.output;

    if opts.verbose {
        println!("Writing image to '{out_file}' ...");
    }

    image
        .save(out_file)
        .map_err(|e| {
            eprintln!("Error saving image to '{}': {}", out_file, e);
            exit(1);
        })
        .unwrap();

    if opts.verbose {
        println!("Write successful!");
    }
}
