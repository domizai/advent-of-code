use std::fs::read_to_string;

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn eval(target: u64, values: &[u64], result: u64) -> bool {
    if values.is_empty() { return result == target; }
    eval(target, &values[1..], result + values[0]) ||
    eval(target, &values[1..], result * values[0]) ||
    eval(target, &values[1..], concat(result, values[0]))
}

fn main() {
    let input = read_to_string("input.txt").unwrap().trim().to_string();
    let sum: u64 = input.lines().map(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        let target: u64 = parts[0].trim().parse().unwrap();
        let values: Vec<u64> = parts[1].split_whitespace().filter_map(|n| n.trim().parse().ok()).collect();
        if eval(target, &values[1..], values[0]) { target } else { 0 }
    }).sum();
    println!("{}", sum);
}
