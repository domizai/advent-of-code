import fs from 'fs';

const example = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`;

const input = fs.readFileSync('input.txt', 'utf8').trim();
const grid = input.split("\n").map((row) => row.trim().split(""));
const word = "XMAS".split("");

const search = (sx, sy) => (dx, dy) => {
    let { x, y, c } = { x: sx, y: sy, c: 0 };
    while (grid[x]?.[y] && word[c]) {
        if (grid[x][y] !== word[c]) break;
        x += dx, y += dy, c++;
    }
    return +(c >= word.length);
};

let count = 0;
for (let y = 0; y < grid[0].length; y++) {
    for (let x = 0; x < grid.length; x++) {
        const s = search(x, y);
        count += s(1,0) + s(-1,0) + s(0,1) + s(0,-1) + s(1,1) + s(-1,-1) + s(1,-1) + s(-1,1);
    }
}

console.log(count);
