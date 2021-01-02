import { Universe } from "rust-wasm-tutorial";
import * as wasm from "rust-wasm-tutorial/rust_wasm_tutorial_bg.wasm";
import { Cell } from "rust-wasm-tutorial/rust_wasm_tutorial_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById('game-of-life-canvas');
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;
let ticksPerAnimation = 1;

const playPauseButton = document.getElementById('play-pause');
const ticksPerAnimationButton = document.getElementById('ticks-per-animation');
const randomInitButton = document.getElementById('random-init');
const killAllButton = document.getElementById('kill-all');

const getIndex = (row, column) => {
    return row * width + column;
};

const bitIsSet = (n, array) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (array[byte] & mask) === mask;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; ++i) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j <= height; ++j) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(wasm.memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
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

const renderLoop = () => {
    for (let count = 0; count < ticksPerAnimation; ++count) {
        universe.tick();
    }

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

canvas.addEventListener('click', event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    if (event.ctrlKey) {
        universe.glider(row, col);
    } else if (event.shiftKey) {
        universe.pulsar(row, col);
    } else {
        universe.toggle_cell(row, col);
    }

    drawGrid();
    drawCells();
});

const isPaused = () => {
    return animationId === null;
};

const play = () => {
    playPauseButton.textContent = '⏸';
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = '▶';
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener('click', event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

ticksPerAnimationButton.addEventListener('change', event => {
    ticksPerAnimation = event.target.value;
});

randomInitButton.addEventListener('click', event => {
    universe.random();
});

killAllButton.addEventListener('click', event => {
    universe.killall();
});

play();
