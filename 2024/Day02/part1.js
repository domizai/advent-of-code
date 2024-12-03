const fs = require('fs');

function testRepost (levels) {
    const direction = Math.sign(levels[1] - levels[0]);
    for (let i = 1; i < levels.length; i++) {
        const diff = levels[i] - levels[i - 1];
        const dist = Math.abs(diff);
        if (dist > 3 || dist < 1) return false;
        if (Math.sign(diff) != direction) return false;
    }
    return true;
}

const example = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`;

const input = fs.readFileSync('input.txt', 'utf8').trim();
const reports = input.split("\n").map(report => report.trim().split(" ").map(n => parseInt(n)));
const count = reports.reduce((acc, levels) => acc + (testRepost(levels) ? 1 : 0), 0);
console.log(count);
