use std::fs::read_to_string;
use std::convert::TryInto;

fn eval(target: u64, values: &[u64], result: u64) -> bool {
    if values.is_empty() { return result == target; }
    eval(target, &values[1..], result + values[0]) ||
    eval(target, &values[1..], result * values[0])
}

fn main() {
    let sum: u64 = read_to_string("input.txt").unwrap().trim().to_string().lines().map(|line| {
        let [left, right]: [&str; 2] = line.split(':').collect::<Vec<&str>>().try_into().unwrap();
        let target: u64 = left.trim().parse().unwrap();
        let values: Vec<u64> = right.split_whitespace().filter_map(|n| n.trim().parse().ok()).collect();
        eval(target, &values[1..], values[0]) as u64 * target
    }).sum();
    println!("{}", sum);
}

