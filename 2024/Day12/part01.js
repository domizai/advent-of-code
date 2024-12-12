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
    if (visited[y][x] !== cell) return;
    if (!regionCellsMap[uid]) regionCellsMap[uid] = [];
    regionCellsMap[uid].push([x, y]);
    visited[y][x] = null;
    [[x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]]
        .filter(([i, j]) => !(i < 0 || i >= rows || j < 0 || j >= cols))
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
    top: 1, right: 4, bottom: 2, left: 8,
    0: 0, 1: 1, 2: 1, 3: 2, 4: 1, 5: 2, 6: 2, 7: 3, 8: 1, 9: 2, 10: 2, 11: 3, 12: 2, 13: 3, 14: 3, 15: 4,    
};

const fences = [];
for (let y = 0; y < rows; y++) {
    fences.push([]);
    for (let x = 0; x < cols; x++) {
        let value = 0;
        const cell = grid[y][x];
        if (cell !== grid[y][x + 1])   value += fence.right;
        if (cell !== grid[y + 1]?.[x]) value += fence.bottom;
        if (cell !== grid[y][x - 1])   value += fence.left;
        if (cell !== grid[y - 1]?.[x]) value += fence.top;
        fences[y].push(value);
    }
}

const sum = Object.entries(regionCellsMap)
    .reduce((acc, [cell, locs]) => acc + locs.length * locs
        .reduce((acc, [x, y]) => acc + fence[fences[y][x]], 0), 0);

console.log(sum);
