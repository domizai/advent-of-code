use std::collections::HashMap;

fn main() {
    let stones: Vec<String> = std::fs::read_to_string("input.txt").unwrap().trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
        
    let mut stone_counts: HashMap<String, i64> = HashMap::new();

    for stone in stones {
        *stone_counts.entry(stone.to_string()).or_insert(0) += 1;
    }

    for _ in 0..25 {
        let mut new_stones: HashMap<String, i64> = HashMap::new();
        
        for (stone, &count) in &stone_counts {
            if stone.len() % 2 == 0 {
                let mid = stone.len() / 2;
                let left = &stone[..mid];
                *new_stones.entry(left.to_string()).or_insert(0) += count;
                
                let right = &stone[mid..];
                let right_int = right.parse::<i64>().unwrap();
                *new_stones.entry(right_int.to_string()).or_insert(0) += count;
                
            } else if stone == "0" {
                *new_stones.entry("1".to_string()).or_insert(0) += count;

            } else {
                let new_stone = (stone.parse::<i64>().unwrap() * 2024).to_string();
                *new_stones.entry(new_stone).or_insert(0) += count;
            }
        }

        stone_counts = new_stones;
    }

    let sum: i64 = stone_counts.values().sum();
    println!("{:?}", sum);
}
