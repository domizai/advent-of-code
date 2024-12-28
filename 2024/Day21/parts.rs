use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point { x: isize, y: isize }
impl From<char> for Point {
    fn from(c: char) -> Self {
        match c {
            '<' => Point { x: -1, y: 0 },
            '>' => Point { x: 1, y: 0 },
            '^' => Point { x: 0, y: -1 },
            'v' => Point { x: 0, y: 1 },
            _ => panic!("Invalid point"),
        }
    }
}

type Keypad = HashMap<char, Point>;

fn main() {
    let numpad: Keypad = to_keybad("789,456,123,#0A");
    let dirpad: Keypad = to_keybad("#^A,<v>");

    let keycodes: Vec<String> = fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut cache: HashMap<String, isize> = HashMap::new();

    let part01: isize = solve_and_sum(&keycodes, &numpad, &dirpad, 2, &mut cache);
    println!("{}", part01);

    let part02: isize = solve_and_sum(&keycodes, &numpad, &dirpad, 25, &mut cache);
    println!("{}", part02);
}

fn solve_and_sum(
    keycodes: &Vec<String>, 
    numpad: &Keypad, 
    dirpad: &Keypad, 
    robots: usize, 
    mut cache: &mut HashMap<String, isize>
) -> isize {
    keycodes.iter().map(|code| {
        let numerical: isize = code.split('A').next().unwrap().parse().unwrap();
        numerical * get_key_presses(&numpad, code, robots, &mut cache, &dirpad)
    }).sum()
}

fn to_keybad(input: &str) -> Keypad {
    input.split(",").enumerate().map(|(i, row)| {
        row.chars().enumerate().map(move |(j, c)| (c, Point { x: j as isize, y: i as isize }))
    }).flatten().collect()
}

fn get_command(input: &Keypad, start: char, end: char) -> Vec<String> {
    if start == end { return vec!["A".to_string()]; }
    let mut queue = vec![(input[&start], String::new())];
    let mut distances = HashMap::new();
    let mut all_paths = Vec::new();

    while let Some((current, path)) = queue.pop() {
        if current == input[&end] {
            all_paths.push(path.clone() + "A");
        }
        if let Some(&dist) = distances.get(&(current.x, current.y)) {
            if dist < path.len() { continue; }
        }
        ['^', '>', 'v', '<'].iter().for_each(|&direction| {
            let dir: Point = direction.into();
            let pos = Point { x: current.x + dir.x, y: current.y + dir.y };
            if input[&'#'] == pos { return; }

            if let Some(_) = input.values().find(|&&button| button == pos) {
                let new_path = path.clone() + &direction.to_string();
                if distances.get(&(pos.x, pos.y)).map_or(true, |&dist| dist >= new_path.len()) {
                    queue.push((pos, new_path.clone()));
                    distances.insert((pos.x, pos.y), new_path.len());
                }
            }
        });
    }
    all_paths.sort_by_key(|a| a.len());
    all_paths
}

fn get_key_presses(
    input: &Keypad, 
    code: &str, 
    robot: usize, 
    cache: &mut HashMap<String, isize>, 
    dirpad: &Keypad
) -> isize {
    let key = format!("{},{}", code, robot);
    if let Some(&result) = cache.get(&key) { return result; }
    let mut current = 'A';
    let mut length = 0;
    
    for c in code.chars() {
        let moves = get_command(input, current, c);
        if robot == 0 {
            length += moves[0].len() as isize;
        } else {
            length += moves.iter()
                .map(|mov| get_key_presses(&dirpad, mov, robot - 1, cache, dirpad))
                .min()
                .unwrap();
        }
        current = c;
    }
    cache.insert(key, length);
    length
}
