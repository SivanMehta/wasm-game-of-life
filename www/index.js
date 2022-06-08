import { Universe } from "hello-wasm-pack";
// Import the WebAssembly memory at the top of the file.
import { memory } from "hello-wasm-pack/hello_wasm_pack_bg";

const SIZE = 96;
const CELL_SIZE = 5; // px
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new(SIZE);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

function getIndex(row, column) {
    return row * width + column;
}

function bitIsSet(n, arr) {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    const isSet = (arr[byte] & mask) === mask;
    return isSet;
}

function drawCells() {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = bitIsSet(idx, cells)
            ? ALIVE_COLOR
            : DEAD_COLOR;

        ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
        );
        }
    }

    ctx.stroke();
};

function renderLoop() {
    drawCells();
    universe.tick();
    requestAnimationFrame(renderLoop)
};

requestAnimationFrame(renderLoop)