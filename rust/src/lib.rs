extern crate wasm_bindgen;
extern crate phf;
extern crate image;
extern crate dyn_clone;

mod cpu;
mod ines;
mod controller;
mod nes;
mod memory;
mod ppu;
mod settings;
mod mapper_0;
mod benchmark;

use wasm_bindgen::prelude::*;

// Imports from the web:
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = log)]
    fn js_log(s: &str);
    #[wasm_bindgen(js_name = getInput)]
    fn js_get_input() -> String;
    #[wasm_bindgen(js_name = updateVideo)]
    fn js_update_video(v: &[u8]);
}

struct WebDriver;

impl benchmark::Driver for WebDriver {
    fn log(&self, s: &str) { js_log(s) }
    fn get_input(&self) -> String { js_get_input() }
    fn update_video(&self, v: &[u8]) { js_update_video(v) }
}
use benchmark::Driver;

// Exports to the web:
// Wasm glue code from https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html

// Called when the module is loaded
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    WebDriver.log("Loaded wasm module.");
    Ok(())
}

#[wasm_bindgen(js_name = loadRom)]
pub fn js_load_rom(file: &[u8]) {
    benchmark::load_rom(file)
}

#[wasm_bindgen(js_name = tick)]
pub fn js_tick() {
    benchmark::tick(&mut WebDriver)
}