pub mod ast;
pub mod constant_evaluator;
pub mod evaluator;
pub mod lexer;
pub mod parser;

/// Compile source code into an expression
#[allow(dead_code)] // TODO: Find a better solution
pub fn compile_source(source: String) -> ast::Expr {
    let lexer = lexer::Lexer::new(source);
    let source = lexer.source();
    let tokens: Vec<_> = lexer.collect();
    parser::parse_tokens(tokens, source.clone(), || ())
}
