const fs = require('fs');
const example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
const input = fs.readFileSync('input.txt', 'utf8').trim();
const regex = /(?:mul\((\d{1,}),(\d{1,})\))|(?:don't\(\))|(?:do\(\))/gi;
const result = input.matchAll(regex);
const computation = Array.from(result).reduce((acc, [m,a,b] = match) => {
    if (m === "don't()") acc.do = false;
    else if (m === "do()") acc.do = true;
    else if (acc.do) acc.sum += parseInt(a) * parseInt(b);
    return acc;
}, { do: true, sum: 0 });
console.log(computation.sum);
