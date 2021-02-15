use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("Hello World from InPage Script");
}

#[wasm_bindgen]
pub fn add(number: u8) -> u8 {
    number + 1
}
