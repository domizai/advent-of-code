#![allow(dead_code)]

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

type Vector = (i32, i32);

fn direction_to_vector(dir: char) -> Vector {
    match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
         _  => (0, 0),
    }
}

fn print_warehoure(warehoure: &Vec<Vec<char>>) {
    for row in warehoure {
        println!("{}", row.iter().collect::<String>());
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn is_pushable(current_pos: Vector, dir: char, map: &Vec<Vec<char>>) -> bool {
    let (dy, dx) = direction_to_vector(dir);
    let next_pos = (current_pos.0 + dy, current_pos.1 + dx);
    let ch = map[current_pos.0 as usize][current_pos.1 as usize];

    match ch {
        '#' => false,
        '[' | ']' if dir == '<' || dir == '>' => 
            is_pushable(next_pos, dir, map),
        '[' | ']' if dir == '^' || dir == 'v' => {
            let offset = if ch == '[' { 1 } else { -1 };
            is_pushable(next_pos, dir, map) &&
            is_pushable((next_pos.0, next_pos.1 + offset), dir, map)
        },
        _ => true,
    }
}

fn push(current_pos: Vector, dir: char, map: &mut Vec<Vec<char>>) -> bool {
    let (dy, dx) = direction_to_vector(dir);
    let next_pos = (current_pos.0 + dy, current_pos.1 + dx);
    let ch = map[current_pos.0 as usize][current_pos.1 as usize];

    match ch {
        '#' => false,
        '.' => true,
        '[' | ']' if dir == '^' || dir == 'v' => {
            let off = if ch == '[' { (0, 1) } else { (0, -1) };
            if !is_pushable(next_pos, dir, map) || !is_pushable((next_pos.0 + off.0, next_pos.1 + off.1), dir, map) {
                return false;
            }
            push(next_pos, dir, map);
            push((next_pos.0 + off.0, next_pos.1 + off.1), dir, map);
            let neighbor = if ch == '[' { ']' } else { '[' };
            map[next_pos.0 as usize][next_pos.1 as usize] = ch;
            map[(next_pos.0 + off.0) as usize][(next_pos.1 + off.1) as usize] = neighbor;
            map[current_pos.0 as usize][current_pos.1 as usize] = '.';
            map[current_pos.0 as usize][(current_pos.1 + off.1) as usize] = '.';
            true
        },
        _ if dir == '<' || dir == '>' => {
            let next_ch = if ch == '[' || ch == ']' { ch } else { dir };
            map[current_pos.0 as usize][current_pos.1 as usize] = next_ch;
            if !is_pushable(next_pos, dir, map) { 
                return false; 
            }
            push(next_pos, dir, map);
            map[next_pos.0 as usize][next_pos.1 as usize] = next_ch;
            map[current_pos.0 as usize][current_pos.1 as usize] = '.';
            true
        },
        _ => { // Up or Down
            map[current_pos.0 as usize][current_pos.1 as usize] = dir;
            if !is_pushable(next_pos, dir, map) { 
                return false; 
            }
            push(next_pos, dir, map);
            map[next_pos.0 as usize][next_pos.1 as usize] = dir;
            map[current_pos.0 as usize][current_pos.1 as usize] = '.';
            true
        }
    }
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .split(format!("{}{}", NEWLINE, NEWLINE).as_str())
        .map(|s| s.to_string())
        .collect();

    let mut map: Vec<Vec<char>> = input[0]
        .replace("#", "##").replace("O", "[]").replace(".", "..").replace("@", "@.")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let moves: Vec<char> = input[1]
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .flatten()
        .collect();

    // get the starting position of the robot
    let mut robot = map.iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter()
            .enumerate()
            .map(move |(j, &cell)| (i, j, cell)))
        .find(|&(_, _, cell)| cell == '@')
        .map(|(i, j, _)| (i as i32, j as i32))
        .unwrap();

    // move things around
    moves.iter().for_each(|&dir| {
        dev_only! {
            clear_console();
            print_warehoure(&map);
            println!("next move: {:?}, {:?}", dir, moves.len()); 
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }

        if push(robot, dir, &mut map) {
            let (dy, dx) = direction_to_vector(dir);
            robot = (robot.0 + dy, robot.1 + dx);
        }
    });

    let sum: i64 = map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter()
            .enumerate()
            .filter_map(move |(x, &cell)| {
                if cell == '[' { Some((y as i64) * 100 + (x as i64)) } else { None }
            })
        ).sum();

    println!("{:?}", sum);
}
