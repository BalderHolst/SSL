use clap::Parser;
use std::fs;

mod cli;
mod lexer;

fn main() {

    let opts = cli::Cli::parse();

    let source = fs::read_to_string(opts.input).unwrap();

    let lexer = lexer::Lexer::new(&source);
    let tokens: Vec<_> = lexer.collect();

    println!("{:?}", tokens);

}
