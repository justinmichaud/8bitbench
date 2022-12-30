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

use memory::*;

pub struct Controller {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,

    strobe: bool,
    count: u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
            strobe: false,
            count: 0,
        }
    }
}

impl Mem for Controller {
    fn read(&mut self, _: &mut Box<dyn Mapper>, _: u16) -> u8 {
        let res = if self.strobe {
            self.a
        }
        else {
            if self.count <= 8 {
                self.count += 1;
            }

            match self.count {
                1 => self.a,
                2 => self.b,
                3 => self.select,
                4 => self.start,
                5 => self.up,
                6 => self.down,
                7 => self.left,
                8 => self.right,
                _ => true,
            }
        };

        if res { 1 } else { 0 }
    }

    fn write(&mut self, _: &mut Box<dyn Mapper>, _: u16, val: u8) {
        self.strobe = val&0b0000001>0;
        self.count = 0;
    }
}