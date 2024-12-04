#![allow(unused_variables)]
use std::fs::read_to_string;

fn main () {
    let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";    

    let input = read_to_string("input.txt").unwrap().trim().to_string();
    let lines: Vec<&str> = input.split("\n").map(|l| l.trim()).collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let size = (grid.len() as i32, grid[0].len() as i32);
    let word = "MAS".chars().collect::<Vec<char>>();

    let search = |sx: i32, sy: i32| {
        let grid = &grid;
        let word = &word;
        move |dx: i32, dy: i32| -> i32 {
            let mut c = 0;
            let mut x = sx; 
            let mut y = sy; 
            while x >= 0 && x < size.0 && y >= 0 && y < size.1 && c < word.len() {
                if grid[x as usize][y as usize] != word[c] { break; }
                x += dx; y += dy; c += 1;
            }
            (c >= word.len()).into()
        }
    };
    
    let mut count = 0;
    let off = word.len() as i32 - 1;
    for y in 0..size.1 {
        for x in 0..size.0 {
            let a = search(x, y)(1,1) + search(x + off, y + off)(-1,-1);
            let b = search(x, y + off)(1,-1) + search(x + off, y)(-1,1);
            match a + b { 2 => count += 1, _ => () }
        }
    }

    println!("{:?}", count);
}

