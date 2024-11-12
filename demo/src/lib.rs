use wasm_bindgen::prelude::*;
use ssl;


const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const PIXEL_WIDTH: usize = 4;

static mut STATIC: [u8; WIDTH * HEIGHT * PIXEL_WIDTH] = [0; WIDTH * HEIGHT * PIXEL_WIDTH];

#[wasm_bindgen]
pub fn canvas_width() -> usize {
    WIDTH
}

#[wasm_bindgen]
pub fn canvas_height() -> usize {
    HEIGHT
}

#[wasm_bindgen]
pub fn get_buffer_ptr() -> *const u8 {
    unsafe { STATIC.as_ptr() }
}

#[wasm_bindgen]
pub fn get_buffer_size() -> usize {
    unsafe { STATIC.len() }
}

pub fn get_index(x: usize, y: usize) -> usize {
    (y * WIDTH + x)*PIXEL_WIDTH
}

#[wasm_bindgen]
pub fn render(code: String) {
    let image = ssl::generate(code, WIDTH as u32, HEIGHT as u32);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = get_index(x, y);
            let pixel = image.get_pixel(x as u32, y as u32);
            unsafe {
                STATIC[index] = pixel[0];
                STATIC[index + 1] = pixel[1];
                STATIC[index + 2] = pixel[2];
                STATIC[index + 3] = pixel[3];
            }
        }
    }
}
