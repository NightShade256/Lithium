import * as rendering from "./rendering.js";
import * as logic from "./logic.js";

// Start the webpage's main loop.
export function mainLoop(scale) {
    // Get reference to the canvas and create a rendering context.
    const canvas = document.getElementById("gameCanvas");

    const ctx = canvas.getContext("2d");

    // Reference to the start and clear button.
    const startButton = document.getElementById("startButton");
    const clearButton = document.getElementById("clearButton");

    // The grid is coloured grey.
    ctx.strokeStyle = "grey";

    // State variables that indicate whether we are
    // currently "playing" the game, and the ID provided
    // by setInterval.
    let isPlaying = false;
    let setIntervalId = null;

    // Initialize cells.
    let cells = new Array(canvas.height / scale);

    for (let i = 0; i < cells.length; i++) {
        cells[i] = new Array(canvas.width / scale);
        cells[i].fill(false);
    }

    // Extremely unpolised implementation to turn on/off
    // cells.
    document.onclick = (mouseEvent) => {
        let x = Math.floor(mouseEvent.clientX / scale);
        let y = Math.floor(mouseEvent.clientY / scale);

        if (y >= cells.length || x >= cells[0].length) {
            return;
        }

        if (cells[y][x] === true) {
            cells[y][x] = false;
        } else {
            cells[y][x] = true;
        }
    };

    // Generate the next generation of cells from the given input.
    function nextGeneration() {
        let nextGen = new Array(canvas.height / scale);

        for (let i = 0; i < cells.length; i++) {
            nextGen[i] = new Array(canvas.width / scale);
            nextGen[i].fill(false);
        }

        for (let i = 0; i < cells.length; i++) {
            for (let j = 0; j < cells[i].length; j++) {
                let neighbors = logic.getNeighbourCount(cells, i, j);

                if (cells[i][j] === true) {
                    if (neighbors < 2 || neighbors > 3) {
                        nextGen[i][j] = false;
                    } else {
                        nextGen[i][j] = true;
                    }
                } else {
                    if (neighbors == 3) {
                        nextGen[i][j] = true;
                    }
                }
            }
        }

        cells = nextGen;
    }

    // Start/Stop implementation.
    startButton.onclick = (_) => {
        isPlaying = isPlaying ? false : true;

        if (isPlaying === true) {
            setIntervalId = setInterval(nextGeneration, 400);
            startButton.innerText = "Stop";
        } else {
            clearInterval(setIntervalId);
            startButton.innerText = "Start";
        }
    };

    // Reset the cell state.
    clearButton.onclick = (_) => {
        cells = new Array(canvas.height / scale);

        for (let i = 0; i < cells.length; i++) {
            cells[i] = new Array(canvas.width / scale);
            cells[i].fill(false);
        }
    };

    // All the actual rendering is done here.
    function renderLoop() {
        for (let i = 0; i < cells.length; i++) {
            for (let j = 0; j < cells[i].length; j++) {
                if (cells[i][j] === true) {
                    ctx.fillStyle = "black";
                } else {
                    ctx.fillStyle = "white";
                }

                rendering.fillCell(ctx, j * scale, i * scale, scale);
            }
        }

        rendering.drawGrid(ctx, canvas.width, canvas.height, scale);

        requestAnimationFrame(renderLoop);
    }

    requestAnimationFrame(renderLoop);
}
