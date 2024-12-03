const fs = require('fs');
const input = fs.readFileSync('input.txt', 'utf8').trim();
const [list1, list2] = input.trim().split('\n').reduce((acc, line) => {
    const [m, a, b] = line.match(/(\d{1,})\s+(\d{1,})/);
    acc[0].push(parseInt(a));
    acc[1].push(parseInt(b));
    return acc;
}, [[],[]]);
const sum = list1.reduce((acc, i) => acc + i * list2.filter(j => j == i).length, 0);
console.log(sum);
