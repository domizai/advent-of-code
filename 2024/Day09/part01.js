import fs from 'fs';

let [capacities, compressed] = [[], []];

fs.readFileSync('input.txt', 'utf8')
    .trim()
    .split('')
    .map(n => parseInt(n))
    .forEach((n, i) => {
    if (i % 2 === 0) {
        compressed.push(Array(n).fill(i / 2));
    } else {
        compressed.push([]);
        capacities.push(n);
    }
});

let [l, r] = [1, compressed.length - 1];

_1: while (l < r) {
    while (compressed[l].length < capacities[(l - 1) / 2]) { 
        if (compressed[r].length == 0)
            if ((r -= 2) < l) 
                break _1;
        compressed[l].push(compressed[r].pop());
    }
    l += 2;
}

const checksum = compressed
    .flat()
    .map((n, i) => i * parseInt(n))
    .reduce((acc, n) => acc + n, 0);

console.log(checksum);
