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

function fetchRom() { return loadFile("./assets/rom.nes") }
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