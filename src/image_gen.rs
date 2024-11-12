use image::{self, Rgb, RgbImage};

use crate::{ast::Expr, evaluator::eval};

pub fn generate_image(expr: Expr, width: u32, height: u32) -> RgbImage {
    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let nx = (x as f64) / (width as f64) * 2.0 - 1.0;
            let ny = (y as f64) / (height as f64) * 2.0 - 1.0;

            let c = eval(&expr, nx, ny);

            let r = (c.r * (u8::MAX as f64)) as u8;
            let g = (c.g * (u8::MAX as f64)) as u8;
            let b = (c.b * (u8::MAX as f64)) as u8;

            image.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    image
}
