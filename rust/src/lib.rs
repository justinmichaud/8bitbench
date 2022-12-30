/*
MIT License

Copyright (c) 2023 Justin Michaud

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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