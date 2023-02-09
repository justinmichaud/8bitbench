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

use cpu::*;
use memory::*;
use controller::*;
use ppu::*;
use mapper_0::*;

pub struct Console {
    pub cpu: Cpu,
    pub chipset: Chipset,
}

pub struct Chipset {
    pub mapper: Box<dyn Mapper>,
    pub mem: Memory,
    pub ppu: Ppu,
    pub controller1: Controller,
    pub controller2: Controller,

    ppu_dma_requested: bool,
    ppu_dma_val: u8,

    ppu_writes_requested: Vec<(u16, u8)>,
}

impl Console {
    pub fn new(prg: Vec<u8>, mut chr: Vec<u8>, mapper: u8, prg_ram_size: usize,
               horiz_mapping: bool) -> Console {
        if chr.len() == 0 {
            chr = vec![0; 8*1024];
        }

        let mut mem = Memory::new();
        let mut mapper = match mapper {
            0 => Box::new(Mapper0::new(prg, prg_ram_size, chr)) as Box<dyn Mapper>,
            _ => panic!("Mapper: {}", mapper)
        };

        Console {
            cpu: Cpu::new(mem.read16(&mut mapper, 0xFFFC)),
            chipset: Chipset {
                mapper: mapper,
                mem: mem,
                ppu: Ppu::new(horiz_mapping),
                ppu_dma_requested: false,
                ppu_dma_val: 0,
                controller1: Controller::new(),
                controller2: Controller::new(),

                ppu_writes_requested: vec![],
            },
        }
    }

    pub fn console_tick(&mut self) {
        let frame_time = 1789773/60;
        while self.cpu.count < frame_time {
            if self.chipset.ppu_dma_requested {
                self.chipset.ppu_dma_requested = false;
                self.chipset.ppu.ppudma(&mut self.chipset.mapper, self.chipset.ppu_dma_val,
                                        &mut self.cpu, &mut self.chipset.mem);
            }

            if self.chipset.ppu_writes_requested.len() > 0 {
                for &(addr, val) in &self.chipset.ppu_writes_requested {
                    self.chipset.ppu.write_main(&mut self.chipset.mapper, addr, val, &self.cpu);
                }
                self.chipset.ppu_writes_requested.clear();
            }

            self.cpu.cpu_tick(&mut self.chipset);
            self.chipset.ppu.ppu_tick(&mut self.cpu, &mut self.chipset.mapper);
        }

        self.cpu.count -= frame_time;
    }

    pub fn prepare_draw(&mut self, canvas: &mut ConsoleImageBuffer) {
        self.chipset.ppu.prepare_draw(&mut self.chipset.mapper);

        let w = self.chipset.ppu.output_canvas.width();
        let h = self.chipset.ppu.output_canvas.height();
        let cw = canvas.width();
        let ch = canvas.height();

        for (x,y,p) in canvas.enumerate_pixels_mut() {
            *p = *self.chipset.ppu.output_canvas.get_pixel(x*w/cw, y*h/ch);
        }
    }
}

impl Chipset {
    pub fn read(&mut self, addr: u16) -> u8 {
        match addr as usize {
            0x2000 ..= 0x2007 => self.ppu.read_main(&mut self.mapper, addr),
            0x2008..=0x3FFF => self.read(mirror_addr(0x2000..=0x2007, 0x2008..=0x3FFF, addr)),
            0x4014 => self.ppu.read_main(&mut self.mapper, addr),
            0x4016 => self.controller1.read(&mut self.mapper, addr),
            0x4000 ..= 0x4017 => 0,
            _ => self.mem.read(&mut self.mapper, addr)
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x2000 ..= 0x2007 => {
                self.ppu_writes_requested.push((addr, val));
            },
            0x2008..=0x3FFF => self.write(mirror_addr(0x2000..=0x2007, 0x2008..=0x3FFF, addr), val),
            0x4014 => {
                self.ppu_dma_requested = true;
                self.ppu_dma_val = val;
            },
            0x4016 => {
                self.controller1.write(&mut self.mapper, addr, val);
                self.controller2.write(&mut self.mapper, addr, val);
            },
            0x4000 ..= 0x4017 => { },
            _ => self.mem.write(&mut self.mapper, addr, val)
        }
    }

    pub fn read16(&mut self, addr: u16) -> u16 {
        self.read(addr) as u16 + ((self.read(addr+1) as u16)<<8)
    }

    fn write16(&mut self, addr: u16, val: u16) {
        self.write(addr, (val&0x00FF) as u8);
        self.write(addr+1, ((val&0xFF00)>>8) as u8);
    }
}