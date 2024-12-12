import fs from 'fs';

let input = fs.readFileSync('input.txt', 'utf8').trim();
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
    t:1, b:2, r:4, l:8,
    // fence-value to number of fences per cell
    0:0, 1:1, 2:1, 3:2, 4:1, 5:2, 6:2, 7:3, 8:1, 9:2, 10:2, 11:3, 12:2, 13:3, 14:3, 15:4,    
};

const fences = [];
for (let y = 0; y < rows; y++) {
    fences.push([]);
    for (let x = 0; x < cols; x++) {
        const cell = grid[y][x];
        fences[y].push(
            fence.r * (cell !== grid[y][x + 1])   |
            fence.b * (cell !== grid[y + 1]?.[x]) |
            fence.l * (cell !== grid[y][x - 1])   |
            fence.t * (cell !== grid[y - 1]?.[x]));
    }
}

// Calculate the number of corners in each region
const regionCornersMap = {};
Object.entries(regionCellsMap).forEach(([region, cells]) => {
    let [c, r, l, t, b] = [0, fence.r, fence.l, fence.t, fence.b];
    cells.forEach(([x, y]) => {
        let n, f = fences[y][x];
        c += !!(f & t && f & r) + 
             !!(f & b && f & l) +
             !!(f & r && f & b) + 
             !!(f & l && f & t) +
             !!((n = fences[y+1]?.[x-1]) && (n & (t | r)) && !(f & (b | l))) +
             !!((n = fences[y-1]?.[x-1]) && (n & (b | r)) && !(f & (t | l))) +
             !!((n = fences[y+1]?.[x+1]) && (n & (t | l)) && !(f & (b | r))) +
             !!((n = fences[y-1]?.[x+1]) && (n & (b | l)) && !(f & (t | r)));
    });
    regionCornersMap[region] = c;
});

// Sum of each region's area multiplied by the number of corners (= number  of sides)
const sum = Object.entries(regionCellsMap)
    .reduce((acc, [region, cells]) => acc + cells.length * regionCornersMap[region], 0);  

console.log(sum);
