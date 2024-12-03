const fs = require('fs');
const example = `3   4
4   3
2   5
1   3
3   9
3   3`;
const input = fs.readFileSync('input.txt', 'utf8').trim();
const [list1,list2] = input.trim().split('\n').reduce((acc, line) => {
    const [m, a, b] = line.match(/(\d{1,})\s+(\d{1,})/);
    acc[0].push(parseInt(a));
    acc[1].push(parseInt(b));
    return acc;
}, [[],[]]).map(list => list.sort());
const sum = list1.reduce((acc, n, i) => acc + Math.abs(n - list2[i]), 0);
console.log(sum);