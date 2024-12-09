import fs from 'fs';

let diskmap = fs.readFileSync('input.txt', 'utf8')
    .trim()
    .split('')
    .map(n => parseInt(n));

let capacities = [];
let compressed = [];

diskmap.forEach((n, i) => {
    if (i % 2 === 0) {
        compressed.push(Array(n).fill(i/2));
    } else {
        compressed.push([]);
        capacities.push(n);
    }
});

let l = 1;                     // pointer to odd blocks to fill
let r = compressed.length - 1; // pointer to even blocks to move
let m = (l - 1) / 2;           // pointer to capacities
 
_1: while (true) { 
    while (                                     // look for matching blocks to move and fill
        compressed[r].length > capacities[m] || // left block needs to have free space
        compressed[r].every(n => n == null)     // right block musn't be moved
    ) {
        if ((r -= 2) < l) { 
            l += 2; // could't find free space, try the next block
            r = compressed.length - 1;
            if ((m = (l - 1) / 2) > capacities.length) // we tried all blocks
                break _1;
        }
    }
    compressed[l].push(...compressed[r]);         // fill the left block
    capacities[m] -= compressed[r].length;        // update capacity
    compressed[r] = compressed[r].map(n => null); // clear the block
    r = compressed.length - 1;                    // reset the pointer
}

// fill the remaining spaces
for (let i = 1; i < compressed.length; i+=2)
    compressed[i].push(...Array(capacities[(i-1)/2]).fill(null));

const checksum = compressed
    .filter(a => a.length > 0)
    .map(a => a.length == 0 ? [0] : a)
    .flat()
    .map((n, i) => n == null ? 0 : i * parseInt(n))
    .reduce((acc, n) => acc + n, 0);

console.log(checksum);
