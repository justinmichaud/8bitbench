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

// Wasm glue code modified from https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
let global = (typeof window !== 'undefined' ? window : (typeof self !== 'undefined' ? self : (0,eval)("this")))

// This can be included as a node module or as an es6 module
if (typeof global.log === 'undefined')
    throw "Global missing log"

if (typeof global.loadFile === 'undefined')
    throw "Global missing loadFile"

if (typeof global.fetchRom === 'undefined')
    throw "Global missing fetchRom"

if (typeof global.video === 'undefined')
    throw "Global missing video"

if (global.video.length != 256 * 240 * 4)
    throw "Length is wrong"

global.getInput = function() {
    return "t"
}

global.updateVideo = function(vec) {
    for (let i = 0; i < global.video.length; ++i) {
        global.video[i] = vec[i]
    }
}

async function run(optionalWasm) {
    const pkg = await import('../rust/pkg/emu_bench.js')
    const init = pkg.default
    const { loadRom, js_tick } = pkg

    await init(optionalWasm)
    let rom = await fetchRom()
    loadRom(rom)

    let frames = 5 * 60
    let start = Date.now()
    for (let i = 0; i < frames; ++i) {
        js_tick()
    }
    let end = Date.now()

    if (global.video.length != 256 * 240 * 4)
        throw "Length is wrong"

    return { ms: end - start, frames }
}

global.run = run