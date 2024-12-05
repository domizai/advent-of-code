use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file").trim().to_string();
    let parts: Vec<&str> = input.split("\n\n").map(|p| p.trim()).collect();
    let [rule_tuples, updates] = parts.as_slice() else { panic!() };

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in rule_tuples.lines() {
        let rule: Vec<i32> = line.split('|').map(|n| n.trim().parse().expect("Invalid number")).collect();
        if let [left, right] = *rule.as_slice() {
            rules.entry(left).or_insert_with(Vec::new).push(right);
        }
    }

    let sum: i32 = updates.lines().map(|line| {
        let mut update: Vec<i32> = line.split(',').map(|n| n.trim().parse().expect("Invalid number")).collect();
        for i in 1..update.len() {
            if rules.get(&update[i]).map_or(false, |rights| {
                rights.iter().any(|&right| update[..i].contains(&right))
            }) {
                update.sort_by(|&a, &b| {
                    if rules.get(&a).map_or(false, |rights| rights.contains(&b)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                return update[(update.len() - 1) / 2];
            }
        }
        0
    }).sum();

    println!("{}", sum);
}
