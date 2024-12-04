import fs from 'fs';

const example = `.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........`;

const input = fs.readFileSync('input.txt', 'utf8').trim();
const grid = input.split("\n").map((row) => row.trim().split(""));
const word = "MAS".split("");

const search = (sx, sy) => (dx, dy) => {
    let { x, y, c } = { x: sx, y: sy, c: 0 };
    while (grid[x]?.[y] && word[c]) {
        if (grid[x][y] !== word[c]) break;
        x += dx, y += dy, c++;
    }
    return +(c >= word.length);
};

let count = 0, off = word.length - 1;
for (let y = 0; y < grid[0].length; y++) {
    for (let x = 0; x < grid.length; x++) {
        let a = search(x, y)(1,1) + search(x + off, y + off)(-1,-1);
        let b = search(x, y + off)(1,-1) + search(x + off, y)(-1,1);
        count += (a + b) > 1;
    }
}

console.log(count);
