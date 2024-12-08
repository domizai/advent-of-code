// rustc part01.rs -C opt-level=3 -o main && ./main && rm main
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
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
    let size = (grid[0].len() as i32, grid.len() as i32);

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for symbols in antennas.values() {
        for &(x, y) in symbols {
            for &(x2, y2) in symbols {
                if (x, y) == (x2, y2) { continue; }
                let diff = (x - x2, y - y2);
                let node = (x + diff.0, y + diff.1);
                if node.0 < 0 || node.0 >= size.0 || node.1 < 0 || node.1 >= size.1 { break; }
                antinodes.insert(node);
            }
        }
    }

    println!("{:?}", antinodes.len());
}
