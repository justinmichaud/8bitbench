const canvas = document.getElementById("canvas")

if (!crossOriginIsolated)
    throw "CORS check failed"

const worker = new Worker("js/worker.js", {
    type: 'module'
})
const sab = new SharedArrayBuffer(4 * 256 * 240);
worker.postMessage(sab);

function update() {
    const imageData = self.ctx.getImageData(0, 0, self.canvas.width, self.canvas.height);
    const data = imageData.data;
    for (let i = 0; i < self.length; ++i) {
        data[i] = sab[i]
    }
    self.ctx.putImageData(imageData, 0, 0);
    requestAnimationFrame(update)
}