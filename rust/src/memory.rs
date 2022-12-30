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

use std::ops::RangeInclusive;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub trait Mapper: DynClone + Debug {
    fn read(&mut self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, val: u8);

    fn read_ppu(&mut self, addr: u16) -> u8;

    fn write_ppu(&mut self, addr: u16, val: u8);

    fn horizontal_mirroring(&self, rom_val: bool) -> bool;
}

pub trait Mem {
    fn read(&mut self, mapper: &mut Box<dyn Mapper>, addr: u16) -> u8;

    fn read16(&mut self, mapper: &mut Box<dyn Mapper>, addr: u16) -> u16 {
        self.read(mapper, addr) as u16 + ((self.read(mapper, addr+1) as u16)<<8)
    }

    fn write(&mut self, mapper: &mut Box<dyn Mapper>, addr: u16, val: u8);

    fn write16(&mut self, mapper: &mut Box<dyn Mapper>, addr: u16, val: u16) {
        self.write(mapper, addr, (val&0x00FF) as u8);
        self.write(mapper, addr+1, ((val&0xFF00)>>8) as u8);
    }
}

pub struct Memory {
    pub ram: [u8; 2 * 1024],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 2048]
        }
    }
}

impl Mem for Memory {
    fn read(&mut self, mapper: &mut Box<dyn Mapper>, addr: u16) -> u8 {
        match addr {
            0..=0x07FF => self.ram[addr as usize],
            0x0800..=0x1FFF => self.read(mapper, mirror_addr(0..=0x07FF, 0x0800..=0x1FFF, addr)),
            0x4020..=0xFFFF => mapper.read(addr),
            _ => {
                panic!("Reference to invalid main address {:X}", addr);
            }
        }
    }

    fn write(&mut self, mapper: &mut Box<dyn Mapper>, addr: u16, val: u8) {
        match addr {
            0..=0x07FF => self.ram[addr as usize] = val,
            0x0800..=0x1FFF => self.write(mapper, mirror_addr(0..=0x07FF, 0x0800..=0x1FFF, addr), val),
            0x4020..=0xFFFF => mapper.write(addr, val),
            _ => {
                panic!("Reference to invalid main address {:X}", addr);
            }
        }
    }
}

pub fn mirror_addr(from : RangeInclusive<u16>, to : RangeInclusive<u16>, addr : u16) -> u16 {
    let size = from.end() - from.start() + 1;

    let offset = (addr - to.start()) % size;
    from.start() + offset
}