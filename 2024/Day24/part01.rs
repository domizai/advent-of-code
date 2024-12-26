use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

pub fn parse_file(path: &str) -> (BTreeMap<String, i32>, Vec<Vec<String>>) {
    let file = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut values_map = BTreeMap::new();
    let mut ops = Vec::new();

    // parse values
    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.is_empty() { break; }
        let parts: Vec<&str> = line.split(&[' ', ':'][..]).filter(|s| !s.is_empty()).collect();
        values_map.insert(parts[0].to_string(), parts[1].parse().unwrap());
    }

    // parse operations
    for line in lines {
        let line = line.unwrap();
        let parts: Vec<String> = line.split(&[' ', '-', '>'][..])
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        ops.push(parts);
    }

    (values_map, ops)
}

fn calculate_z_appended(map: &BTreeMap<String, i32>) -> i64 {
    let mut result = String::new();
    for (key, value) in map.iter() {
        if !key.is_empty() && key.starts_with('z') {
            result.insert(0, char::from_digit(*value as u32, 10).unwrap());
        }
    }
    i64::from_str_radix(&result, 2).unwrap()
}

fn main() {
    let (mut values, mut ops) = parse_file("input.txt");

    while !ops.is_empty() {
        let current_op = ops.remove(0);
        let var1 = &current_op[0];
        let var2 = &current_op[2];

        if values.contains_key(var1) && values.contains_key(var2) {
            let result = match current_op[1].as_str() {
                "AND" => values[var1] & values[var2],
                "OR" => values[var1] | values[var2],
                "XOR" => values[var1] ^ values[var2],
                _ => 0,
            };
            values.insert(current_op[3].clone(), result);
        } else {
            ops.push(current_op);
        }
    }

    let result = calculate_z_appended(&values);
    println!("{}", result);
}