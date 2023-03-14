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

#[derive(Debug)]
pub struct Flags {
    prg_size: usize,
    chr_size: usize,
    pub prg_ram_size: usize,
    pub horiz_mirroring: bool,
}

pub fn load_file(contents: &[u8]) -> (Flags, Vec<u8>, Vec<u8>) {
    let flags = Flags {
        prg_size: contents[0] as usize * 16384,
        chr_size: contents[1] as usize * 8192,
        prg_ram_size: 8192 as usize,
        horiz_mirroring: true,
    };

    let prg = &contents[16..(16+flags.prg_size)];
    let chr = &contents[(16+flags.prg_size)..(16+flags.prg_size+flags.chr_size)];

    (flags, prg.to_vec(), chr.to_vec())
}