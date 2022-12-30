// The main driving code for both native and web versions

use nes::Nes;
use ppu;
use ppu::NesImageBuffer;
use ines;

struct App {
    nes: Nes,
    canvas: NesImageBuffer,
    video: Vec<u8>,
}

static mut APP: Option<App> = None;

pub trait Driver {
    fn log(&self, s: &str);
    fn get_input(&self) -> String;
    fn update_video(&self, v: &[u8]);
}

pub fn load_rom(file: &[u8]) {
    let (flags, prg, chr) = ines::load_file(file);
    let nes = Nes::new(prg, chr, flags.mapper, flags.prg_ram_size, flags.horiz_mirroring);
    let canvas = ppu::make_canvas(256, 240);
    unsafe {
        APP = Some(App { nes, canvas, video: Vec::new() });
    }
}

pub fn tick(driver: &mut dyn Driver) {
    let input = driver.get_input();

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
        driver.update_video(&app.video.as_slice());
    }
}