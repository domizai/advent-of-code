#![allow(unused_variables)]
#![allow(dead_code)]
use std::fs::read_to_string;
use std::collections::HashSet;

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

// rustc part02.rs -o main && ./main && rm main
fn main() {
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
    let mut grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let size = (grid.len() as i32, grid[0].len() as i32);
    let cells = size.0 * size.1;
    let dirs:[(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let syms:[char; 4] = ['^', '>', 'v', '<'];
    
    let mut pos = (0, 0);
    let mut dir_index = 0;
    'a: for y in 0..size.1 {
        for x in 0..size.0 {
            let c = &grid[y as usize][x as usize];
            if syms.contains(c) {
                pos = (x, y);
                dir_index = syms.iter().position(|&s| s == *c).unwrap();
                break 'a;
            }
        }
    }

    let time = std::time::Instant::now();
    let mut count = 0;
    for j in 0..size.1 {
        for i in 0..size.0 {
            let cell = &mut grid[j as usize][i as usize];
            if *cell == '#' { continue; }
            *cell = '#';

            clear_console();
            println!("{:.2}%, {:?}s", (j * size.0 + i) as f32 / cells as f32 * 100.0, time.elapsed().as_secs());

            let (mut x, mut y) = pos;
            let mut dir_index = dir_index;
            let mut dir = dirs[dir_index];
            let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

            while x >= 0 && x < size.0 && y >= 0 && y < size.1 {
                if visited.contains(&((x, y), dir)) {
                    count += 1;
                    break;
                }
                if grid[y as usize][x as usize] == '#' {
                    x -= dir.0;
                    y -= dir.1;
                    dir_index = (dir_index + 1) % 4;
                    dir = dirs[dir_index];
                }
                visited.insert(((x, y), dir));
                x += dir.0;
                y += dir.1;
            }
            visited.clear();
            grid[j as usize][i as usize] = '.';
        }
    }

    println!("{:?}", count);
}

