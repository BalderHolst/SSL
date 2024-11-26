//! A library for generating images from a random input. This create uses the `RgbImage` struct from the `image` crate to represent images.
//!
//! # Simple Example
//! ```
//! use ssl::{create_image, generate};
//!
//! // Some random string (or deliberate code)
//! let source = "Hello, I am a random string!".to_string();
//!
//! // Generate an image from the source
//! let image = generate(source, 600, 600);
//!
//! // Save image to a file (uses the `image` crate)
//! image.save("output.png").unwrap();
//!
//! ```
//!
//! # Render in Parts
//! Sometimes you want to call a function every so often while rendering an image. Usually to report rendering progress. This can be achieved with the [render_in_parts] function.
//! ```
//! use ssl::{create_image, parse_source, render_in_parts};
//!
//! // Some random string (or deliberate code)
//! let source = "Hello, I am a *different* random string!".to_string();
//!
//! // Compile source code into an expression
//! let expr = parse_source(source);
//!
//! // Render image in 10 parts, printing the progress
//! let parts = 10;
//! let image = render_in_parts(&expr, 600, 600, parts, |n| {
//!    println!("Rendered {}% ...", (n+1) * 100 / parts);
//! });
//!
//! // Save image to a file (uses the `image` crate)
//! image.save("output.png").unwrap();
//! ```
//!
//! # Multiple Threads
//! Rendering an image can be a slow process. To speed things up, you can render parts of the image in parallel using multiple threads. This can be achieved with the [render_subimage] function.

#![warn(missing_docs)]

mod compiler;
mod renderer;

/// Create an image to be rendered into
pub fn create_image(width: u32, height: u32) -> image::RgbImage {
    image::RgbImage::new(width, height)
}

pub use image::RgbImage;

pub use renderer::{render, render_in_parts, render_into, render_part_into, render_subimage};

pub use compiler::parser::parse_source;

/// Generate an image from a source string
pub fn generate(source: String, width: u32, height: u32) -> image::RgbImage {
    let expr = parse_source(source);
    renderer::render(&expr, width, height)
}
