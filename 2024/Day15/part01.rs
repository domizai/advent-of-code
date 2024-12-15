#![allow(unused_variables, dead_code)]
use std::collections::HashMap;
use std::ops::Add;

fn print_warehoure(warehoure: &Vec<Vec<char>>) {
    for row in warehoure {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct EVec (i32, i32);
impl Add for EVec {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        EVec(self.0 + other.0, self.1 + other.1)
    }
}

fn push (current_pos: EVec, dir: EVec, warehouse: &mut Vec<Vec<char>>) -> bool {
    let next_pos = current_pos + dir;
    let ch = warehouse[current_pos.0 as usize][current_pos.1 as usize];
    let next_ch = warehouse[next_pos.0 as usize][next_pos.1 as usize];
    if next_ch == '#' { return false; }
    if next_ch == '.' || push(next_pos, dir, warehouse) {
        warehouse[next_pos.0 as usize][next_pos.1 as usize] = ch;
        warehouse[current_pos.0 as usize][current_pos.1 as usize] = '.';
        return true;
    }
    false
}

fn main() {
    let input: Vec<&str> = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<".split("\n\n").map(|s| s.trim()).collect();
    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim().split("\n\n").map(|s| s.trim().to_string()).collect();

    let mut warehouse: Vec<Vec<char>> = input[0].lines().map(|line| line.chars().collect()).collect();
    let mut moves: Vec<char> = input[1].lines().map(|line| line.chars().collect::<Vec<_>>()).flatten().rev().collect();
    let dires: HashMap<char, EVec> = HashMap::from([
        ('^', EVec(-1, 0)),
        ('v', EVec(1, 0)),
        ('<', EVec(0, -1)),
        ('>', EVec(0, 1)),
    ]);

    // get the starting position of the robot
    let mut robot = EVec(0, 0);
    'a: for (i, row) in warehouse.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == '@' {
                robot = EVec(i as i32, j as i32);
                break 'a;
            }
        }
    }

    // move things around
    while !moves.is_empty() {
        let dir = dires[&moves.pop().unwrap()];
        if push(robot, dir, &mut warehouse) {
            robot = robot + dir;
        }
    }

    let mut sum: i64 = 0;
    for y in 1..warehouse.len() - 1 {
        for x in 1..warehouse[0].len() - 1 {
            if warehouse[y][x] == 'O' {
                sum += (y as i64) * 100 + (x as i64);
            }
        }
    }
    println!("{:?}", sum);
}
