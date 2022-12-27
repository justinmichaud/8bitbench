// Wasm glue code from https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
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

use wasm_bindgen::prelude::*;

use nes::Nes;
use ppu::NesImageBuffer;

struct App {
    nes: Nes,
    canvas: NesImageBuffer,
    video: Vec<u8>,
}

static mut APP: Option<App> = None;

// Imports from the web:
#[wasm_bindgen]
extern "C" {
    fn log(s: &str);
    fn get_input() -> String;
    fn update_video(v: &[u8]);
}

// Exports to the web:

// Called when the module is loaded
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    log("Loaded wasm module.");
    Ok(())
}

#[wasm_bindgen]
pub fn load_rom(file: &[u8]) {
    let (flags, prg, chr) = ines::load_file(file);
    let nes = nes::Nes::new(prg, chr, flags.mapper, flags.prg_ram_size, flags.horiz_mirroring);
    let canvas = ppu::make_canvas(256, 240);
    unsafe {
        APP = Some(App { nes, canvas, video: Vec::new() });
    }
    log("Initialized emulator");
}

#[wasm_bindgen]
pub fn tick() {
    let input = get_input();

    unsafe {
        let app = &mut APP.as_mut().unwrap();

        app.nes.chipset.controller1.up = false;
        app.nes.chipset.controller1.left = false;
        app.nes.chipset.controller1.down = false;
        app.nes.chipset.controller1.right = false;
        app.nes.chipset.controller1.a = false;
        app.nes.chipset.controller1.b = false;
        app.nes.chipset.controller1.start = false;
        app.nes.chipset.controller1.select = false;

        for c in input.chars() {
            match c {
                'u' => app.nes.chipset.controller1.up = true,
                'l' => app.nes.chipset.controller1.left = true,
                'd' => app.nes.chipset.controller1.down = true,
                'r' => app.nes.chipset.controller1.right = true,
                'a' => app.nes.chipset.controller1.a = true,
                'b' => app.nes.chipset.controller1.b = true,
                't' => app.nes.chipset.controller1.start = true,
                'e' => app.nes.chipset.controller1.select = true,
                _ => { }
            }
        }

        app.nes.tick();
        app.nes.prepare_draw(&mut app.canvas);

        app.video = app.canvas.as_raw().to_vec();
        update_video(&app.video.as_slice());
    }
}