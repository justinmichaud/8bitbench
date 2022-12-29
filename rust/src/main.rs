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
mod settings;
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
    let rom = std::fs::read("../assets/rom.nes").unwrap();
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