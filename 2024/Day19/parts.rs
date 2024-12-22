#![allow(unused_variables)]

use std::collections::HashSet;

#[cfg(windows)]
const NEWLINE: &'static str = "\r\n";
#[cfg(not(windows))]
const NEWLINE: &'static str = "\n";

// dynamic programming approach
fn find(patterns: &HashSet<&str>, design: &str) -> usize {
    let n = design.len();
    // dp[i] stores the number of ways to decompose the suffix design[i..]
    let mut dp = vec![0; n + 1]; 
    // base case: one way to decompose an empty string
    dp[n] = 1; 

    for i in (0..n).rev() {
        for &pattern in patterns {
            if design[i..].starts_with(pattern) {
                dp[i] += dp[i + pattern.len()];
            }
        }
    }
    dp[0] // count for the entire string
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .split(format!("{}{}", NEWLINE, NEWLINE).as_str())
        .map(|s| s.to_string())
        .collect();

    let designs: HashSet<&str> = input[1].lines().collect();
    let patterns: HashSet<&str> = input[0].split(", ").collect();
    
    let mut combinations_sum: usize = 0;
    let mut matches_sum: usize = 0;

    for design in &designs {
        let combinations = find(&patterns, design);
        combinations_sum += combinations;
        matches_sum += combinations.min(1);
    }
    println!("matches: {:?}", matches_sum);
    println!("combinations: {:?}", combinations_sum);
}
