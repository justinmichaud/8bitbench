async function doRun() {

function log(str) {
    if (typeof console != 'undefined')
        console.log(str)
    else
        print(str)
}
globalObject.log = log

globalObject.getInput = function() {
    return "t"
}

globalObject.updateVideo = function(vec) {
    for (let i = 0; i < globalObject.video.length; ++i) {
        globalObject.video[i] = vec[i]
    }
}

let ab = new ArrayBuffer(Int16Array.BYTES_PER_ELEMENT * 4 * 256 * 240)
globalObject.video = new Int16Array(ab)

let start = benchmarkTime()
if (!Module.wasmBinary.length || !Module.romBinary.length)
    throw new Error("Needed binary not loaded");

await init(Module.wasmBinary)
loadRom(Module.romBinary)
tick()
reportCompileTime(benchmarkTime() - start)

let frames = 40 * 60
for (let i = 0; i < frames; ++i) {
    tick()
}

if (video.length != 256 * 240 * 4)
    throw "Length is wrong"

// TODO: verify output here

reportRunTime(benchmarkTime() - start)
}