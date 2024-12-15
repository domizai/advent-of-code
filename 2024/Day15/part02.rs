#![allow(unused_variables, dead_code)]
use std::ops::Add;

// step through: rustc part02.rs -o  main && ./main && rm main
// result only:  rustc part02.rs -Oo main && ./main && rm main

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
    match mov {
        Move::Left(_) | Move::Right(_) => {
            match ch {
                '#' => false,
                '[' | ']' => is_pushable(next_pos, mov, warehouse) ,
                _ => true,
            }
        },
        Move::Up(_) | Move::Down(_) => {
            match ch {
                '#' => false,
                '['|']' => {
                    is_pushable(next_pos, mov, warehouse) && 
                    is_pushable(next_pos + EVec(0, if ch == '[' { 1 } else { -1 }), mov, warehouse)
                },
                _ => true,
            }
        }
    }       
}

fn push(current_pos: EVec, mov: Move, warehouse: &mut Vec<Vec<char>>) -> bool {
    let ch = warehouse[current_pos.0 as usize][current_pos.1 as usize];
    let next_pos = current_pos + mov.into();
    match mov {
        Move::Left(_) | Move::Right(_) => {
            match ch {
                '#' => false,
                '.' => true,
                _ => {
                    let next_ch = if ch == '[' || ch == ']' { ch } else { mov.into() };
                    warehouse[current_pos.0 as usize][current_pos.1 as usize] = next_ch;
                    if !is_pushable(next_pos, mov.into(), warehouse) { return false; }
                    push(next_pos, mov, warehouse);
                    warehouse[next_pos.0 as usize][next_pos.1 as usize] = next_ch;
                    warehouse[current_pos.0 as usize][current_pos.1 as usize] = '.';
                    true
                }
            }
        },
        Move::Up(_) | Move::Down(_) => {
            match ch {
                '#' => false,
                '.' => true,
                '['|']' => {
                    let neighbor = if ch == '[' { ']' } else { '[' };
                    let off = if ch == '[' { EVec(0, 1) } else { EVec(0, -1) };
                    if !is_pushable(next_pos, mov.into(), warehouse) || 
                       !is_pushable(next_pos + off, mov.into(), warehouse) {
                        return false;
                    }
                    push(next_pos, mov, warehouse);
                    push(next_pos + off, mov, warehouse);
                    warehouse[next_pos.0 as usize][next_pos.1 as usize] = ch;
                    warehouse[next_pos.0 as usize][(next_pos.1 + off.1) as usize] = neighbor;
                    warehouse[current_pos.0 as usize][current_pos.1 as usize] = '.';
                    warehouse[current_pos.0 as usize][(current_pos.1 + off.1) as usize] = '.';
                    true
                },
                _ => {
                    warehouse[current_pos.0 as usize][current_pos.1 as usize] = mov.into();
                    if !is_pushable(next_pos, mov.into(), warehouse) { return false; }
                    push(next_pos, mov, warehouse);
                    warehouse[next_pos.0 as usize][next_pos.1 as usize] = mov.into();
                    warehouse[current_pos.0 as usize][current_pos.1 as usize] = '.';
                    true
                }
            }
        }
    }
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim().split("\n\n").map(|s| s.trim().to_string()).collect();
    let warehouse_orig: Vec<Vec<char>> = input[0].lines().map(|line| line.chars().collect()).collect();
    let mut moves: Vec<Move> = input[1].lines()
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<_>>()).flatten().rev().collect();

    let mut warehouse: Vec<Vec<char>> = Vec::new();
    for row in warehouse_orig {
        let mut new_row = Vec::new();
        for cell in row {
            match cell {
                '@' => { new_row.push('@'); new_row.push('.') },
                'O' => { new_row.push('['); new_row.push(']') },
                _ => { new_row.push(cell); new_row.push(cell) },
            }
        }
        warehouse.push(new_row);
    }

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
        let mov = moves.pop().unwrap();

        dev_only! {
            clear_console();
            print_warehoure(&warehouse);
            println!("next move: {:?}, {:?}", Into::<char>::into(mov), moves.len()); 
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }

        if push(robot, mov, &mut warehouse) {
            let dir = mov.into();
            robot = robot + dir;
        }
    }

    let mut sum: i64 = 0;
    for y in (1..warehouse.len() - 1).step_by(1) {
        for x in (1..warehouse[0].len() - 1).step_by(1) {
            if warehouse[y][x] == '[' {
                sum += (y as i64) * 100 + (x as i64);
            }
        }
    }

    println!("{:?}", sum);
}
