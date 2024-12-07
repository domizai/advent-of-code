use std::fs::read_to_string;

fn eval(target: u64, values: &[u64], result: u64) -> bool {
    if values.is_empty() { return result == target; }
    eval(target, &values[1..], result + values[0]) ||
    eval(target, &values[1..], result * values[0])
}

fn main() {
    let sum: u64 = read_to_string("input.txt").unwrap().trim().to_string().lines().map(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        let target: u64 = parts[0].trim().parse().unwrap();
        let values: Vec<u64> = parts[1].split_whitespace().filter_map(|n| n.trim().parse().ok()).collect();
        eval(target, &values[1..], values[0]) as u64 * target
    }).sum();
    println!("{}", sum);
}

