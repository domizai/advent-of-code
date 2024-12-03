const fs = require('fs');
const example = 'xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))';
const input = fs.readFileSync('input.txt', 'utf8').trim();
const regex = /mul\((\d{1,}),(\d{1,})\)/gi;
const result = input.matchAll(regex);
const sum = Array.from(result).reduce((acc, [s,a,b] = match) => acc + parseInt(a) * parseInt(b), 0);
console.log(sum);
