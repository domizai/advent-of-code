#![allow(unused_variables)]
#![allow(dead_code)]
use std::fs::read_to_string;
use std::collections::HashSet;
use std::thread;
use std::sync::Arc;

fn print(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

// rustc part02_threaded.rs -C opt-level=3 -o main && ./main && rm main
fn main() {
    let time = std::time::Instant::now();
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";    

    let input = read_to_string("input.txt").unwrap().trim().to_string();
    let lines: Vec<&str> = input.split("\n").map(|l| l.trim()).collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let size = (grid.len() as i32, grid[0].len() as i32);
    let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let syms: [char; 4] = ['^', '>', 'v', '<'];

    let mut pos = (0, 0);
    let mut dir_index = 0;
    'a: for y in 0..size.0 {
        for x in 0..size.1 {
            let c = grid[y as usize][x as usize];
            if syms.contains(&c) {
                pos = (x, y);
                dir_index = syms.iter().position(|&s| s == c).unwrap();
                break 'a;
            }
        }
    }

    let mut handles = Vec::new();
    let grid = Arc::new(grid);
    for j in 0..size.0 {
        for i in 0..size.1 {
            if grid[j as usize][i as usize] == '#' { continue; }
            let wall = (i as i32, j as i32);
            let grid = Arc::clone(&grid); 
            handles.push(thread::spawn(move || {
                let (mut x, mut y) = pos;
                let mut dir_index = dir_index;
                let mut dir = dirs[dir_index];
                let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
                while x >= 0 && x < size.1 && y >= 0 && y < size.0 {
                    if visited.contains(&((x, y), dir)) { return 1; }
                    if grid[y as usize][x as usize] == '#' || (x, y) == wall {
                        x -= dir.0;
                        y -= dir.1;
                        dir_index = (dir_index + 1) % 4;
                        dir = dirs[dir_index];
                    }
                    visited.insert(((x, y), dir));
                    x += dir.0;
                    y += dir.1;
                }
                0 
            }));
        }
    }

    let mut count = 0;
    for handle in handles {
        count += handle.join().unwrap();
    }
    println!("{:?}s", time.elapsed().as_secs());
    println!("{:?}", count);
}

