#![allow(unused_variables, dead_code)]
use std::ops::Add;

// step through: rustc part02.rs -o  main && ./main && rm main
// result only:  rustc part02.rs -Oo main && ./main && rm main

#[cfg(windows)]
const NEWLINE: &'static str = "\r\n";
#[cfg(not(windows))]
const NEWLINE: &'static str = "\n";

macro_rules! dev_only {
    ($($body:tt)*) => {
        #[cfg(debug_assertions)] {
            $($body)*
        }
    };
}

fn print_warehoure(warehoure: &Vec<Vec<char>>) {
    for row in warehoure {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
enum Move { Up(EVec),Down(EVec),Left(EVec),Right(EVec) } 
impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            '^' => Move::Up(EVec(-1, 0)),
            'v' => Move::Down(EVec(1, 0)),
            '<' => Move::Left(EVec(0, -1)),
            '>' => Move::Right(EVec(0, 1)),
            _ => panic!("Invalid move"),
        }
    }
}

impl From<Move> for char {
    fn from(m: Move) -> Self {
        match m {
            Move::Up(_)    => '^',
            Move::Down(_)  => 'v',
            Move::Left(_)  => '<',
            Move::Right(_) => '>',
        }
    }
}

impl From<Move> for EVec {
    fn from(m: Move) -> Self {
        match m {
            Move::Up(v)    => v,
            Move::Down(v)  => v,
            Move::Left(v)  => v,
            Move::Right(v) => v,
        }
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

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn is_pushable(current_pos: EVec, mov: Move, warehouse: &Vec<Vec<char>>) -> bool {
    let next_pos = current_pos + mov.into();
    let ch = warehouse[current_pos.0 as usize][current_pos.1 as usize];
    match ch {
        '#' => false,
        '[' | ']' if matches!(mov, Move::Left(_) | Move::Right(_)) => 
            is_pushable(next_pos, mov, warehouse),
        '[' | ']' if matches!(mov, Move::Up(_) | Move::Down(_)) => {
            let offset = if ch == '[' { 1 } else { -1 };
            is_pushable(next_pos, mov, warehouse) &&
            is_pushable(next_pos + EVec(0, offset), mov, warehouse)
        },
        _ => true,
    }
}

fn push(current_pos: EVec, mov: Move, warehouse: &mut Vec<Vec<char>>) -> bool {
    let ch = warehouse[current_pos.0 as usize][current_pos.1 as usize];
    let next_pos = current_pos + mov.into();
    match ch {
        '#' => false,
        '.' => true,
        '[' | ']' if matches!(mov, Move::Up(_) | Move::Down(_)) => {
            let off = if ch == '[' { EVec(0, 1) } else { EVec(0, -1) };
            if !is_pushable(next_pos, mov.into(), warehouse) || !is_pushable(next_pos + off, mov.into(), warehouse) {
                return false;
            }
            push(next_pos, mov, warehouse);
            push(next_pos + off, mov, warehouse);
            let neighbor = if ch == '[' { ']' } else { '[' };
            warehouse[next_pos.0 as usize][next_pos.1 as usize] = ch;
            warehouse[next_pos.0 as usize][(next_pos.1 + off.1) as usize] = neighbor;
            warehouse[current_pos.0 as usize][current_pos.1 as usize] = '.';
            warehouse[current_pos.0 as usize][(current_pos.1 + off.1) as usize] = '.';
            true
        },
        _ if matches!(mov, Move::Left(_) | Move::Right(_)) => {
            let next_ch = if ch == '[' || ch == ']' { ch } else { mov.into() };
            warehouse[current_pos.0 as usize][current_pos.1 as usize] = next_ch;
            if !is_pushable(next_pos, mov.into(), warehouse) { 
                return false; 
            }
            push(next_pos, mov, warehouse);
            warehouse[next_pos.0 as usize][next_pos.1 as usize] = next_ch;
            warehouse[current_pos.0 as usize][current_pos.1 as usize] = '.';
            true
        },
        _ => { // Up or Down
            warehouse[current_pos.0 as usize][current_pos.1 as usize] = mov.into();
            if !is_pushable(next_pos, mov.into(), warehouse) { 
                return false; 
            }
            push(next_pos, mov, warehouse);
            warehouse[next_pos.0 as usize][next_pos.1 as usize] = mov.into();
            warehouse[current_pos.0 as usize][current_pos.1 as usize] = '.';
            true
        }
    }
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .split(format!("{}{}", NEWLINE, NEWLINE).as_str())
        .map(|s| s.trim().to_string())
        .collect();

    let warehouse_orig: Vec<Vec<char>> = input[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut moves: Vec<Move> = input[1]
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<_>>())
        .flatten()
        .rev()
        .collect();

    let mut warehouse: Vec<Vec<char>> = warehouse_orig
        .iter()
        .map(|row| row.iter()
            .flat_map(|&cell| match cell {
                '@' => vec!['@', '.'],
                'O' => vec!['[', ']'],
                 _  => vec![cell, cell],
            }).collect()
        ).collect();

    // get the starting position of the robot
    let mut robot = warehouse
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter()
            .enumerate()
            .map(move |(j, &cell)| (i, j, cell)))
        .find(|&(_, _, cell)| cell == '@')
        .map(|(i, j, _)| EVec(i as i32, j as i32))
        .unwrap();

    // move things around
    while let Some(mov) = moves.pop() {
        dev_only! {
            clear_console();
            print_warehoure(&warehouse);
            println!("next move: {:?}, {:?}", Into::<char>::into(mov), moves.len()); 
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }

        if push(robot, mov, &mut warehouse) {
            robot = robot + mov.into();
        }
    }

    let sum: i64 = warehouse
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == '[' { Some((y as i64) * 100 + (x as i64)) } else { None }
            })
        })
    .sum();

    println!("{:?}", sum);
}
