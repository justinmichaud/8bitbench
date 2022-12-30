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

function fetchRom() { return loadFile("../assets/nesdoug/full_game.nes") }
self.fetchRom = fetchRom

async function runWithSAB(sab) {
    self.video = new Int16Array(sab)
    await import('./bench.js')
    postMessage(await self.run())
}

addEventListener('message', e => {
    runWithSAB(e.data)
})
