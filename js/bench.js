// Wasm glue code from https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
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
    const { loadRom, tick } = pkg

    await init(optionalWasm)
    let rom = await fetchRom()
    loadRom(rom)

    let frames = 5 * 60
    let start = Date.now()
    for (let i = 0; i < frames; ++i) {
        tick()
    }
    let end = Date.now()

    if (global.video.length != 256 * 240 * 4)
        throw "Length is wrong"

    return { ms: end - start, frames }
}

global.run = run