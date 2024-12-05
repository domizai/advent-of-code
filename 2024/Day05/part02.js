import fs from 'fs';

let input = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`;

input = fs.readFileSync('input.txt', 'utf8').trim();
let [rules, updates] = input.split("\n\n").map(p => p.trim());

rules = rules.split('\n').map(p => p.split('|').map(n => parseInt(n)))
    .reduce((acc, [left, right] = rule) => {
        if (!acc[left]) acc[left] = [];
        acc[left].push(right);
        return acc;
    }, {});

const sum = updates.split("\n").map(p => p.trim().split(',').map(n => parseInt(n)))
    .reduce((sum, update) => {
        for (let i = 1; i < update.length; i++) {
            if (!rules[update[i]]) continue;
            if (rules[update[i]].some((right) => update.slice(0, i).includes(right))) {
                sum += update.sort((a, b) => rules[a]?.includes(b) ? -1 : 1)[(update.length -1) / 2];
                break;
            }
        }
        return sum;
    }, 0);

console.log(sum);
