import { FPS } from "hello-wasm-pack";

export default class FramesPerSecond {
    constructor() {
        this.target = document.getElementById('fps');
        this.fps = new FPS();
    }

    render() {
        // if we haven't yet gathered enough data
        const output = this.fps.render();
        if(!this.fps.enough()) return;
        const content = ["avg", "min", "max"].map(fn => {
            const fps = output[fn]().toPrecision(4);
            return `${fn}: ${fps}`;
        }).join('\n').trim();

        this.target.innerHTML = content;
    }
}