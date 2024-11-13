#![allow(dead_code)] // TODO: Find a better solution

use image::{self, Rgb, RgbImage};

use crate::{ast::Expr, evaluator::eval};

fn f64_color_to_u8(color: f64) -> u8 {
    (color * (u8::MAX as f64)) as u8
}

fn normalize(x: u32, y: u32, width: u32, height: u32) -> (f64, f64) {
    let nx = (x as f64) / (width as f64) * 2.0 - 1.0;
    let ny = (y as f64) / (height as f64) * 2.0 - 1.0;
    (nx, ny)
}

/// Render a part of an expression into a provided image
pub fn render_part_into(image: &mut RgbImage, expr: &Expr, part: u32, total_parts: u32) {
    let start_y = part * image.height() / total_parts;
    let end_y = (part + 1) * image.height() / total_parts;
    for y in start_y..end_y {
        for x in 0..image.width() {
            let (nx, ny) = normalize(x, y, image.width(), image.height());
            let c = eval(expr, nx, ny);
            image.put_pixel(
                x,
                y,
                Rgb([
                    f64_color_to_u8(c.r),
                    f64_color_to_u8(c.g),
                    f64_color_to_u8(c.b),
                ]),
            );
        }
    }
}

/// Render an expression into an image in parts, calling the provided function after each part
pub fn render_in_parts(
    expr: &Expr,
    width: u32,
    height: u32,
    total_parts: u32,
    f: impl Fn(u32),
) -> RgbImage {
    let mut image = RgbImage::new(width, height);
    for part in 0..total_parts {
        render_part_into(&mut image, expr, part, total_parts);
        f(part);
    }
    image
}

/// Render an expression into a provided image
pub fn render_into(image: &mut RgbImage, expr: &Expr) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let (nx, ny) = normalize(x, y, image.width(), image.height());
            let c = eval(expr, nx, ny);
            image.put_pixel(
                x,
                y,
                Rgb([
                    f64_color_to_u8(c.r),
                    f64_color_to_u8(c.g),
                    f64_color_to_u8(c.b),
                ]),
            );
        }
    }
}

/// Render an expression into an image
pub fn render(expr: &Expr, width: u32, height: u32) -> RgbImage {
    let mut image = RgbImage::new(width, height);
    render_into(&mut image, expr);
    image
}
