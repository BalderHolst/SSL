use wasm_bindgen::prelude::*;


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
pub fn render() {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let index = get_index(x, y);
            unsafe {
                STATIC[index+0] = 255;
                STATIC[index+1] = 100;
                STATIC[index+2] = 0;
                STATIC[index+3] = 255;
            }
        }
    }
}
