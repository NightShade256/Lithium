// Draw a grid of a certain size on the provided canvas context.
export function drawGrid(ctx, w, h, scale) {
    for (let x = 0; x < w; x += scale) {
        for (let y = 0; y < h; y += scale) {
            ctx.strokeRect(x, y, scale, scale);
        }
    }
}

// Fill in the cell that corresponds to the given coordinates and scale.
export function fillCell(ctx, x, y, scale) {
    ctx.fillRect(x * scale, y * scale, scale, scale);
}
