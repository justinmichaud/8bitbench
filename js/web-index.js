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