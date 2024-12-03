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

function testRepostTolerate (levels) {
    return levels.reduce((acc, level, j) => {
        acc.push(levels.filter((n,i) => i != j));
        return acc;
    }, [levels]).map(report => testRepost(report)).some(result => result);
}

const input = fs.readFileSync('input.txt', 'utf8').trim();
const reports = input.split("\n").map(report => report.trim().split(" ").map(n => parseInt(n)));
const count = reports.reduce((acc, levels) => acc + (testRepostTolerate(levels) ? 1 : 0), 0);
console.log(count);
