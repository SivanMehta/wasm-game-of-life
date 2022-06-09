import { FPS } from "hello-wasm-pack";

export default class FramesPerSecond {
    constructor() {
        this.target = document.getElementById('fps');
        this.rustTarget = document.getElementById('fps-rust');
        this.fps = new FPS();
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // save only the last 100 timings
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        let min = Infinity;
        let max = 0;
        let sum = 0;
        this.frames.forEach(frame => {
            sum += frame;
            min = Math.min(min, frame);
            max = Math.max(max, frame);
        });

        let avg = sum / this.frames.length;

        this.target.textContent = `
        Frames Per Second:
            rolling average (last 100) = ${Math.round(avg)}
            rolling max (last 100)     = ${Math.round(max)}
            rolling min (last 100)     = ${Math.round(min)}
        `.trim();

        const output = this.fps.render();
        this.rustTarget.textContent = JSON.stringify('what');
    }
}