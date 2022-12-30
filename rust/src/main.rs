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

use std::{time::SystemTime};

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
struct NativeDriver;

static mut BUFFER: Vec<u8> = vec![];

impl benchmark::Driver for NativeDriver {
    fn log(&self, s: &str) { println!("{}", s) }
    fn get_input(&self, ) -> String { "t".into() }
    fn update_video(&self, v: &[u8]) { unsafe { BUFFER.copy_from_slice(v) } }
}

pub fn native_load_rom(file: &[u8]) {
    unsafe {
        BUFFER = vec![0; 4 * 256 * 240];
    }
    benchmark::load_rom(file);
}

pub fn native_tick() {
    benchmark::tick(&mut NativeDriver)
}

pub fn main() {
    let rom = std::fs::read("../assets/nesdoug/full_game.nes").unwrap();
    native_load_rom(&rom);

    let frames = 5 * 60;
    let start = SystemTime::now();
    for i in 0..frames {
        native_tick();
    }
    let end = SystemTime::now();

    unsafe {
        if BUFFER.len() != 256 * 240 * 4 {
            panic!("Length is wrong")
        }
    }

    println!("Average ms/frame: {}", (end.duration_since(start).expect("").as_millis() as f64) / (frames as f64));
}