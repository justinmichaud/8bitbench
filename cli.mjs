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
let self = (0,eval)("this")
let isNode = typeof global !== 'undefined'
let isJSC = !isNode

function log(str) {
    if (typeof console != 'undefined')
        console.log(str)
    else
        print(str)
}
self.log = log

function loadFile(path) {
    return new Promise ((resolve) => {
        if (isNode) {
            import('fs').then((fs) => {
                const binary = fs.readFileSync(path)
                let byteArray = new Uint8Array(binary)
                log("Got file " + path + " with length: " + byteArray.length)
                resolve(byteArray)
            })
            return
        }
        let byteArray = read(path, 'binary')
        log("Got file " + path + " with length: " + byteArray.length)
        resolve(byteArray)
    })
}
self.loadFile = loadFile

function fetchRom() { return loadFile("./assets/nesdoug/full_game.nes") }
self.fetchRom = fetchRom

async function runCLI() {
    let ab = new ArrayBuffer(Int16Array.BYTES_PER_ELEMENT * 4 * 256 * 240)
    self.video = new Int16Array(ab)

    await import('./lib/fast-text-encoding-1.0.3/text.js')
    await import('./js/bench.js')
    let { ms, frames } = await self.run(loadFile('./rust/pkg/emu_bench_bg.wasm'))
    log("Emulation completed with average ms per frame of " + ((0.0 + ms) / frames))
}

await runCLI()