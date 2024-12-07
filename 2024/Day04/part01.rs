#![allow(unused_variables)]
use std::fs::read_to_string;

fn main () {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";    

    let input = read_to_string("input.txt").unwrap().trim().to_string();
    let lines: Vec<&str> = input.split("\n").map(|l| l.trim()).collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let size = (grid.len() as i32, grid[0].len() as i32);
    let word = "XMAS".chars().collect::<Vec<char>>();

    let search = |sx: i32, sy: i32| {
        let (grid, word) = (&grid, &word);
        move |dx: i32, dy: i32| -> i32 {
            let (mut c, mut x, mut y) = (0, sx, sy);
            while x >= 0 && x < size.0 && y >= 0 && y < size.1 && c < word.len() {
                if grid[x as usize][y as usize] != word[c] { break; }
                (x, y, c) = (x + dx, y + dy, c + 1);
            }
            (c >= word.len()).into()
        }
    };
    
    let mut count = 0;
    for y in 0..size.1 {
        for x in 0..size.0 {
            let s = search(x, y);
            count += s(1,0) + s(-1,0) + s(0,1) + s(0,-1) + s(1,1) + s(-1,-1) + s(1,-1) + s(-1,1);
        }
    }

    println!("{:?}", count);
}

