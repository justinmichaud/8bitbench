// Wasm glue code from https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
import init, { tick, load_rom } from '../rust/pkg/emu_bench.js'

if (!crossOriginIsolated)
    throw "CORS check failed"

function log(str) {
    if (typeof window !== undefined)
        console.log(str)
    else
        print(str)
}

self.log = log

self.get_input = function() {
    return "t"
}

self.update_video = function(vec) {
    for (let i = 0; i < self.buffer.length; ++i) {
        self.buffer[i] = vec[i]
    }
}

function fetchRom() {
    return new Promise ((resolve) => {
        const req = new XMLHttpRequest()
        req.open("GET", "/assets/rom.nes", true)
        req.responseType = "arraybuffer"

        req.onload = (event) => {
            const arrayBuffer = req.response
            if (arrayBuffer) {
                const byteArray = new Uint8Array(arrayBuffer)
                log("Got rom with length: " + byteArray.length)
                resolve(byteArray)
            } else {
                throw "Unable to fetch input ROM"
            }
        }

        req.send(null)
    })
}

async function run() {
    await init()
    let rom = await fetchRom()
    load_rom(rom)

    let frames = 0
    while (true) {
        tick()
        ++frames
    }
}

addEventListener('message', e => {
    self.buffer = e.data.buffer

    run()
})