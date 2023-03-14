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

// The main driving code for both native and web versions

use console::Console;
use ppu;
use ppu::ConsoleImageBuffer;
use binloader;

struct App {
    console: Console,
    canvas: ConsoleImageBuffer,
    video: Vec<u8>,
}

static mut APP: Option<App> = None;

pub trait Driver {
    fn log(&self, s: &str);
    fn get_input(&self) -> String;
    fn update_video(&self, v: &[u8]);
}

pub fn load_rom(file: &[u8]) {
    let (flags, prg, chr) = binloader::load_file(file);
    let console = Console::new(prg, chr, 0, flags.prg_ram_size, flags.horiz_mirroring);
    let canvas = ppu::make_canvas(256, 240);
    unsafe {
        APP = Some(App { console, canvas, video: Vec::new() });
    }
}

pub fn bench_tick(driver: &mut dyn Driver) {
    let input = driver.get_input();

    unsafe {
        let app = &mut APP.as_mut().unwrap();

        app.console.chipset.controller1.up = false;
        app.console.chipset.controller1.left = false;
        app.console.chipset.controller1.down = false;
        app.console.chipset.controller1.right = false;
        app.console.chipset.controller1.a = false;
        app.console.chipset.controller1.b = false;
        app.console.chipset.controller1.start = false;
        app.console.chipset.controller1.select = false;

        for c in input.chars() {
            match c {
                'u' => app.console.chipset.controller1.up = true,
                'l' => app.console.chipset.controller1.left = true,
                'd' => app.console.chipset.controller1.down = true,
                'r' => app.console.chipset.controller1.right = true,
                'a' => app.console.chipset.controller1.a = true,
                'b' => app.console.chipset.controller1.b = true,
                't' => app.console.chipset.controller1.start = true,
                'e' => app.console.chipset.controller1.select = true,
                _ => { }
            }
        }

        app.console.console_tick();
        app.console.prepare_draw(&mut app.canvas);

        app.video = app.canvas.as_raw().to_vec();
        driver.update_video(&app.video.as_slice());
    }
}