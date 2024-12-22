// rustc part01.rs -Oo main && ./main
// to enable dev_only! macro and show maze:
// rustc part01.rs -o main && ./main

#![allow(dead_code)]

mod solver;
use solver::{Pos, solve, backtrace};
use std::collections::HashSet;

const SIZE: usize = 71;
const BYTES: usize = 1024;

fn print_maze(walls: &Vec<Pos>, path: &HashSet<Pos>) {
    let mut maze: Vec<Vec<String>> = vec![vec![format!("\x1B[1;30m{}\x1B[0m", '.'); SIZE]; SIZE];
    for pos in walls {
        maze[pos.1][pos.0] = '#'.to_string();
    }
    for pos in path {
        maze[pos.1][pos.0] = format!("\x1B[1;31m{}\x1B[0m", 'O');
    }
    maze.iter().for_each(|row| println!("{}", row.join("")));
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let corrupted: Vec<Pos> = input.trim().lines().map(|s| {
        let pos: Vec<usize> = s.split(",").map(|s| s.parse().unwrap()).collect();
        Pos (pos[0], pos[1])
    }).collect();

    let start = Pos (0, 0);
    let end = Pos (SIZE - 1, SIZE - 1);
    let bytes = &corrupted[0..BYTES].to_vec();
    let map = solve(&bytes, start, &end, (SIZE, SIZE));
    let path = backtrace(&map, end);

    dev_only! { print_maze(&bytes, &path); }
    println!("Path steps: {}", path.len() - 1);
}