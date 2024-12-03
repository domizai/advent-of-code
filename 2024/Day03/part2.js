const fs = require('fs');
const example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
const input = fs.readFileSync('input.txt', 'utf8').trim();
const result = input.matchAll(/(?:mul\((?<a>\d{1,}),(?<b>\d{1,})\))|(?<dont>don't\(\))|(?<doit>doit\(\))/gi);
const computation = Array.from(result).reduce((acc, { groups: { a, b, dont, doit }} = match) => {
    if (dont) acc.do = false;
    else if (doit) acc.do = true;
    else if (acc.do) acc.sum += parseInt(a) * parseInt(b);
    return acc;
}, { do: true, sum: 0 });
console.log(computation.sum);
