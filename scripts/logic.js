// Get the number of live neighbours a particular cell has.
// Extremely unpolished.
export function getNeighbourCount(currentGen, y, x) {
    let count = 0;

    if (y - 1 >= 0 && x - 1 >= 0) {
        count += currentGen[y - 1][x - 1] ? 1 : 0;
    }

    if (y - 1 >= 0) {
        count += currentGen[y - 1][x] ? 1 : 0;
    }

    if (y - 1 >= 0 && x + 1 < currentGen[0].length) {
        count += currentGen[y - 1][x + 1] ? 1 : 0;
    }

    if (x - 1 >= 0) {
        count += currentGen[y][x - 1] ? 1 : 0;
    }

    if (x + 1 < currentGen[0].length) {
        count += currentGen[y][x + 1] ? 1 : 0;
    }

    if (y + 1 < currentGen.length && x - 1 >= 0) {
        count += currentGen[y + 1][x - 1] ? 1 : 0;
    }

    if (y + 1 < currentGen.length) {
        count += currentGen[y + 1][x] ? 1 : 0;
    }

    if (y + 1 < currentGen.length && x + 1 < currentGen[0].length) {
        count += currentGen[y + 1][x + 1] ? 1 : 0;
    }

    return count;
}
