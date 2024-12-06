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

    // find the starting position and direction
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
    let pos_start = pos;
    let dir_start = dir_index;

    // first, trace the path
    let mut trace: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = pos_start;
    let mut dir_index = dir_start;
    let mut dir = dirs[dir_index];

    while x >= 0 && x < size.0 && y >= 0 && y < size.1 {
        if grid[y as usize][x as usize] == '#' {
            (x, y) = (x - dir.0, y - dir.1);
            dir_index = (dir_index + 1) % 4;
            dir = dirs[dir_index];
        }
        trace.insert((x, y));
        (x, y) = (x + dir.0, y + dir.1);
    }

    let mut handles = Vec::new();
    let grid = Arc::new(grid);

    for p in trace.iter() {
        let wall = *p;
        let grid = Arc::clone(&grid); 

        handles.push(thread::spawn(move || {
            let (mut x, mut y) = pos_start;
            let mut dir_index = dir_start;
            let mut dir = dirs[dir_index];
            let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

            while x >= 0 && x < size.1 && y >= 0 && y < size.0 {
                if visited.contains(&((x, y), dir)) { return 1; }
                if grid[y as usize][x as usize] == '#' || (x, y) == wall {
                    (x, y) = (x - dir.0, y - dir.1);
                    dir_index = (dir_index + 1) % 4;
                    dir = dirs[dir_index];
                }
                visited.insert(((x, y), dir));
                (x, y) = (x + dir.0, y + dir.1);
            }
            0 
        }));
    }

    let mut count = 0;
    for handle in handles {
        count += handle.join().unwrap();
    }
    println!("{:?}ms", time.elapsed().as_millis());
    println!("{:?}", count);
}

