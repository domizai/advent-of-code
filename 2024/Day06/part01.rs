#![allow(unused_variables)]
#![allow(dead_code)]
use std::fs::read_to_string;

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}

// rustc part01.rs -o main && ./main && rm main
fn main () {
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
    let size = (grid[0].len() as i32, grid.len() as i32);
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

    let (mut x, mut y) = pos;
    let mut dir = dirs[dir_index];
    let mut count = 0;
    while x >= 0 && x < size.0 && y >= 0 && y < size.1 {
        if grid[y as usize][x as usize] == '#' {
            (x, y) = (x - dir.0, y - dir.1);
            dir_index = (dir_index + 1) % 4;
            dir = dirs[dir_index];
        }
        let cell = &mut grid[y as usize][x as usize];
        if *cell != 'X' {
            *cell = 'X';
            count += 1;
        }
        (x, y) = (x + dir.0, y + dir.1);
    }

    println!("{:?}", count);
}

