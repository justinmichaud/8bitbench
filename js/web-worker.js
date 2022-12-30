if (!crossOriginIsolated)
    throw "CORS check failed"

function log(str) {
    console.log(str)
}
self.log = log

function loadFile(path) {
    return new Promise ((resolve) => {
        const req = new XMLHttpRequest()
        req.open("GET", path, true)
        req.responseType = "arraybuffer"

        req.onload = (event) => {
            const arrayBuffer = req.response
            if (arrayBuffer) {
                const byteArray = new Uint8Array(arrayBuffer)
                log("Got file " + path + " with length: " + byteArray.length)
                resolve(byteArray)
            } else {
                throw "Unable to fetch input ROM"
            }
        }

        req.send(null)
    })
}
self.loadFile = loadFile

function fetchRom() { return loadFile("../assets/rom.nes") }
self.fetchRom = fetchRom

async function runWithSAB(sab) {
    self.video = new Int16Array(sab)
    await import('./bench.js')
    postMessage(await self.run())
}

addEventListener('message', e => {
    runWithSAB(e.data)
})
