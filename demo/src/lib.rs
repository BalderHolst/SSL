mod sizes;

use sizes::{DEFAULT_DIM, DEFAULT_HEIGHT, DEFAULT_WIDTH, IMAGE_SIZES, MAX_SIZE};
use wasm_bindgen::prelude::*;

const PIXEL_WIDTH: u32 = 4;

static mut DIM: (usize, usize) = DEFAULT_DIM;
static mut STATIC: [u8; (MAX_SIZE * PIXEL_WIDTH) as usize] = [0; (MAX_SIZE * PIXEL_WIDTH) as usize];

// console.log
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

pub fn canvas_size() -> (u32, u32) {
    unsafe {
        let (r, i) = DIM;
        (|| IMAGE_SIZES.get(r)?.1.get(i).cloned())().unwrap_or((DEFAULT_WIDTH, DEFAULT_HEIGHT))
    }
}

#[wasm_bindgen]
pub fn canvas_width() -> u32 {
    canvas_size().0
}

#[wasm_bindgen]
pub fn canvas_height() -> u32 {
    canvas_size().1
}

#[wasm_bindgen]
pub fn canvas_aspect_ratio() -> usize {
    unsafe { DIM.0 }
}

#[wasm_bindgen]
pub fn aspect_ratio_strings() -> Vec<String> {
    IMAGE_SIZES
        .iter()
        .map(|(name, _)| name.to_string())
        .collect()
}

#[wasm_bindgen]
pub fn dim_strings(aspect_ratio: usize) -> Vec<String> {
    IMAGE_SIZES
        .get(aspect_ratio)
        .map(|(_, sizes)| sizes.iter().map(|(w, h)| format!("{}x{}", w, h)).collect())
        .unwrap_or_default()
}

#[wasm_bindgen]
pub fn canvas_resolution() -> usize {
    unsafe { DIM.1 }
}

#[wasm_bindgen]
pub fn get_buffer_ptr() -> *const u8 {
    unsafe { STATIC.as_ptr() }
}

#[wasm_bindgen]
pub fn get_buffer_size() -> u32 {
    let (width, height) = canvas_size();
    width * height * PIXEL_WIDTH
}

fn set_size(aspect_ratio: usize, size_index: usize) {
    if aspect_ratio >= IMAGE_SIZES.len() {
        console_log!("Invalid aspect ratio: {}", aspect_ratio);
        return;
    }
    if size_index >= IMAGE_SIZES[aspect_ratio].1.len() {
        console_log!("Invalid size index: {}", size_index);
        return;
    }
    unsafe {
        DIM = (aspect_ratio, size_index);
    }
}

pub fn get_index(x: u32, y: u32, width: u32) -> usize {
    ((y * width + x) * PIXEL_WIDTH) as usize
}

#[wasm_bindgen]
pub fn render(code: String, aspect_ratio: usize, size_index: usize) {
    set_size(aspect_ratio, size_index);

    let expr = ssl::parse_source(code);

    let (width, height) = canvas_size();
    let image = ssl::render(&expr, width, height);

    for y in 0..height {
        for x in 0..width {
            let index = get_index(x, y, width);
            let pixel = image.get_pixel(x, y);
            unsafe {
                STATIC[index] = pixel[0];
                STATIC[index + 1] = pixel[1];
                STATIC[index + 2] = pixel[2];
                STATIC[index + 3] = u8::MAX;
            }
        }
    }
}
