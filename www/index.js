import { Universe } from "hello-wasm-pack";
// Import the WebAssembly memory at the top of the file.
import { memory } from "hello-wasm-pack/hello_wasm_pack_bg";

const SIZE = 64;
const CELL_SIZE = 10; // px
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const GRID_COLOR = '#CCCCCC';

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

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
};

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

let animationId;
const playPauseButton = document.getElementById('play-pause');

function playing() {
    return animationId !== null;
}

function play() {
    animationId = requestAnimationFrame(renderLoop);
    playPauseButton.textContent = "Pause";
}

function pause() {
    cancelAnimationFrame(animationId);
    playPauseButton.textContent = "Play";
    animationId = null;
}

playPauseButton.addEventListener('click', () => {
    // cancel animation events to "pause"
    if (playing()) {
        pause();
    } else {
        play();
    }
});

function renderLoop() {
    drawCells();
    universe.tick();
    animationId = requestAnimationFrame(renderLoop)
};

canvas.addEventListener('click', event => {
    if(playing()) return;
    const boundingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.floor(canvasTop / CELL_SIZE);
    const col = Math.floor(canvasLeft / CELL_SIZE);
    
    console.log(row, col);
    universe.toggle_cell(row, col);
    drawCells();
})

drawGrid();
play();