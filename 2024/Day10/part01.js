import fs from 'fs';

let input = `89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732`;

input = fs.readFileSync('input.txt', 'utf8').trim();
const grid = input.split('\n').map(row => row.split('').map(n => parseInt(n)));
const [rows, cols] = [grid.length, grid[0].length];

const trailheads = [];
for (let r = 0; r < rows; r++)
    for (let c = 0; c < cols; c++)
        if (grid[r][c] === 0)
            trailheads.push([c, r]);

const find = (p, dest) => {
    if (grid[p[1]][p[0]] === 9){
        dest.add(JSON.stringify(p));
        return;
    }
    [[p[0]-1, p[1]],[p[0]+1, p[1]],[p[0], p[1]-1],[p[0], p[1]+1]]
        .filter(n => !(n[0] < 0 || n[0] >= cols || n[1] < 0 || n[1] >= rows))
        .filter(n => grid[n[1]][n[0]] - grid[p[1]][p[0]] === 1)
        .forEach(n => find(n, dest));
}

const sum = trailheads.reduce((acc, n) => {
    const destinations = new Set();
    find(n, destinations);
    return acc + destinations.size;
}, 0);

console.log(sum);
