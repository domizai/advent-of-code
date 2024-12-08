#![allow(unused_variables)]
use std::fs::read_to_string;
use std::collections::{HashSet, HashMap}; 

fn main() {
    let input ="............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let input = read_to_string("input.txt").unwrap().trim().to_string();
    let lines: Vec<&str> = input.split("\n").map(|l| l.trim()).collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let size = (grid.len() as i32, grid[0].len() as i32);
    let symbols: HashSet<char> = input.chars().filter(|c| c.is_alphabetic() || c.is_numeric()).collect();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = symbols.iter().map(|s| (*s, Vec::new())).collect();

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if symbols.contains(&c) {
                antennas.get_mut(&c).unwrap().push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for symbols in antennas.values() {
        for &(x, y) in symbols {
            for &(x2, y2) in symbols {
                if (x, y) == (x2, y2) { continue; }
                let diff = (x - x2, y - y2);
                for i in 0.. {
                    let node = (x + diff.0 * i, y + diff.1 * i);
                    if node.0 < 0 || node.0 >= size.0 || node.1 < 0 || node.1 >= size.1 { break; }
                    antinodes.insert(node);
                }
            }
        }
    }

    println!("{:?}", antinodes.len());
}
