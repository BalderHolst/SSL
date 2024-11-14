//! A library for generating images from a random input.

#![warn(missing_docs)]

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod renderer;
mod text;

/// Create an image to be rendered into
pub fn create_image(width: u32, height: u32) -> image::RgbImage {
    image::RgbImage::new(width, height)
}

pub use renderer::{render, render_in_parts, render_into, render_part_into};

pub use parser::parse_source;

/// Generate an image from a source string
pub fn generate(source: String, width: u32, height: u32) -> image::RgbImage {
    let expr = parse_source(source);
    renderer::render(&expr, width, height)
}
