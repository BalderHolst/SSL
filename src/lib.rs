mod ast;
mod evaluator;
mod image_gen;
mod lexer;
mod parser;
mod text;

pub fn generate(source: String, width: u32, height: u32) -> image::RgbImage {
    let lexer = lexer::Lexer::new(source);
    let source = lexer.source();
    let tokens: Vec<_> = lexer.collect();

    let mut parser = parser::Parser::new(tokens, source.clone());
    let expr = parser.parse_expr();

    image_gen::generate_image(expr, width, height)
}
