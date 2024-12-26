#[cfg(windows)]
const NEWLINE: &'static str = "\r\n";
#[cfg(not(windows))]
const NEWLINE: &'static str = "\n";

fn convert_to_height_count(arr: &Vec<Vec<char>>) -> Vec<i32> {
    let mut h_count = vec![0; arr[0].len()];
    for col in 0..arr[0].len() {
        for row in 0..arr.len() {
            if arr[row][col] == '#' {
                h_count[col] += 1;
            }
        }
    }
    h_count.iter().map(|x| x - 1).collect()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let combs_f: Vec<&str> = contents.split(format!("{}{}", NEWLINE, NEWLINE).as_str()).collect();

    let mut combs: Vec<Vec<Vec<char>>> = Vec::new();
    for c in combs_f {
        let co: Vec<Vec<char>> = c
            .lines()
            .map(|l| l.trim().chars().collect())
            .collect();
        combs.push(co);
    }

    let mut locks: Vec<Vec<i32>> = Vec::new();
    let mut keys: Vec<Vec<i32>> = Vec::new();

    for c in combs {
        if !c.is_empty() && !c[0].is_empty() && c[0][0] == '#' {
            locks.push(convert_to_height_count(&c));
        } else {
            keys.push(convert_to_height_count(&c));
        }
    }

    let mut ans = 0;

    for lock in &locks {
        for key in &keys {
            let mut works = true;
            for i in 0..key.len() {
                if key[i] + lock[i] > 5 {
                    works = false;
                    break;
                }
            }
            if works { ans += 1; }
        }
    }

    println!("Answer: {}", ans);
}