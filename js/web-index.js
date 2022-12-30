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

const canvas = document.getElementById("canvas")
let ctx = canvas.getContext("2d")
const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

if (!crossOriginIsolated)
    throw "CORS check failed"

const worker = new Worker("js/web-worker.js", {
    type: 'module'
})
const sab = new SharedArrayBuffer(Int16Array.BYTES_PER_ELEMENT * 4 * 256 * 240);
let arr = new Int16Array(sab)
worker.onmessage = ({ data: { frames, ms } }) => {
    alert("Emulation completed with average ms per frame of " + ((0.0 + ms) / frames))
}
worker.postMessage(sab);

function update() {
    const data = imageData.data;
    for (let i = 0; i < arr.length; ++i) {
        data[i] = arr[i]
    }
    ctx.putImageData(imageData, 0, 0)
    requestAnimationFrame(update)
}
update()