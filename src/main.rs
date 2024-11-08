use clap::Parser;
use image_gen::generate_image;
use std::{fs, process::exit, rc::Rc};

mod cli;
mod lexer;
mod parser;
mod ast;
mod evaluator;
mod image_gen;
mod text;

fn main() {

    let opts = cli::Cli::parse();

    let source = match fs::read_to_string(&opts.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", opts.input.display(), e);
            exit(1);
        },
    };

    let lexer = lexer::Lexer::new(source);
    let source = lexer.source();
    let tokens: Vec<_> = lexer.collect();

    let mut parser = parser::Parser::new(tokens, source.clone());
    let expr = parser.parse_expr();

    if opts.print_ast {
        expr.print(source);
        exit(0);
    }

    generate_image(expr, opts.width, opts.height, &opts.output);

}
