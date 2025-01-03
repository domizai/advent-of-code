import fs from 'fs';

let input = `RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE`.trim();

input = fs.readFileSync('input.txt', 'utf8').trim();
const grid = input.split('\n').map(row => row.split(''));
const [cols, rows] = [grid[0].length, grid.length];

// Assign each cell to a region
const regionCellsMap = {}; 
const visited = grid.map(row => row.slice());

const floodfill = (x, y, cell, uid) => {
    if (visited[y]?.[x] !== cell) return;
    visited[y][x] = null;
    (regionCellsMap[uid] ||= []).push([x, y]);
    [[x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]]
        .forEach(([i, j]) => floodfill(i, j, cell, uid));
};

for (let y = 0, uid = 0; y < rows; y++) {
    for (let x = 0; x < cols; x++) {
        if (!visited[y][x]) continue;
        floodfill(x, y, visited[y][x], uid++);
    }
}

// Calculate fence-values for each cell
const fence = {  
    top: 1, bottom: 2, right: 4, left: 8,
    // fence-value to number of fences per cell
    0:0, 1:1, 2:1, 3:2, 4:1, 5:2, 6:2, 7:3, 8:1, 9:2, 10:2, 11:3, 12:2, 13:3, 14:3, 15:4,    
};

const fences = [];
for (let y = 0; y < rows; y++) {
    fences.push([]);
    for (let x = 0; x < cols; x++) {
        const cell = grid[y][x];
        fences[y].push(
            fence.right  * (cell !== grid[y][x + 1])   |
            fence.bottom * (cell !== grid[y + 1]?.[x]) |
            fence.left   * (cell !== grid[y][x - 1])   |
            fence.top    * (cell !== grid[y - 1]?.[x]));
    }
}

// Sum of each region's area multiplied by the number of fences
const sum = Object.entries(regionCellsMap)
    .reduce((acc, [cell, cells]) => acc + cells.length * cells
        .reduce((acc, [x, y]) => acc + fence[fences[y][x]], 0), 0);

console.log(sum);
