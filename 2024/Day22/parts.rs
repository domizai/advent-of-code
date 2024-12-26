use std::collections::HashMap;
use std::fs;

fn main() {
    let secrets: Vec<usize> = fs::read_to_string("input.txt").unwrap()
        .trim()
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let sum: usize = secrets.iter().map(|&s| {
        (0..2000).fold(s, |res, _| evolve_secret_number(res))
    }).sum();

    println!("{}", sum);

    let mut diffs: Vec<Vec<isize>> = vec![vec![]; secrets.len()];
    let mut cache: HashMap<String, (isize, std::collections::HashSet<usize>)> = HashMap::new();
    let mut max = 0;

    let mut secrets = secrets.clone();
    for _ in 0..2000 {
        secrets = secrets.iter().enumerate().map(|(j, &prev)| {
            let next = evolve_secret_number(prev);

            diffs[j].push((next % 10) as isize - (prev % 10) as isize);
            if diffs[j].len() == 4 {
                let key = diffs[j].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
                let value = cache.entry(key.clone()).or_insert((0, std::collections::HashSet::new()));

                if !value.1.contains(&j) {
                    value.1.insert(j);
                    value.0 += (next % 10) as isize;
                    max = max.max(value.0);
                }
                diffs[j].remove(0);
            }
            next
        }).collect();
    }

    println!("{}", max);
}

fn evolve_secret_number(mut secret_number: usize) -> usize {
    let mix = |value, current| current ^ value;  // Bitwise XOR
    let prune = |current| current % 16777216;    // Modulo 2^24

    let value_step1 = secret_number * 64;
    secret_number = mix(value_step1, secret_number);
    secret_number = prune(secret_number);

    let value_step2 = secret_number / 32;
    secret_number = mix(value_step2, secret_number);
    secret_number = prune(secret_number);

    let value_step3 = secret_number * 2048;
    secret_number = mix(value_step3, secret_number);
    secret_number = prune(secret_number);

    secret_number
}