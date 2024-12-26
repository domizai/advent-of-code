use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point { x: isize, y: isize }

fn main() {
    let numpad = to_keybad("789,456,123,X0A");
    let dirpad = to_keybad("X^A,<v>");

    let bfsdir: HashMap<char, Point> = [
        ('^', Point { x: 0, y: -1 }),
        ('>', Point { x: 1, y: 0 }),
        ('v', Point { x: 0, y: 1 }),
        ('<', Point { x: -1, y: 0 }),
    ].iter().cloned().collect();

    let keycodes: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut memo = HashMap::new();

    let part01: isize = keycodes.iter().map(|code| {
        let numerical: isize = code.split('A').next().unwrap().parse().unwrap();
        numerical * get_key_presses(&numpad, &bfsdir, code, 2, &mut memo, &dirpad)
    }).sum();

    println!("{}", part01);

    let part02: isize = keycodes.iter().map(|code| {
        let numerical: isize = code.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
        numerical * get_key_presses(&numpad, &bfsdir, code, 25, &mut memo, &dirpad)
    }).sum();

    println!("{}", part02);
}

fn to_keybad(input: &str) -> HashMap<char, Point> {
    input.split(",").enumerate().map(|(i, row)| {
        row.chars().enumerate().map(move |(j, c)| (c, Point { x: j as isize, y: i as isize }))
    }).flatten().collect()
}

fn get_command(input: &HashMap<char, Point>, bfsdir: &HashMap<char, Point>, start: char, end: char) -> Vec<String> {
    let mut queue = vec![(input[&start], String::new())];
    let mut distances = HashMap::new();

    if start == end { return vec!["A".to_string()]; }

    let mut all_paths = Vec::new();
    while let Some((current, path)) = queue.pop() {
        if current == input[&end] {
            all_paths.push(path.clone() + "A");
        }
        if let Some(&dist) = distances.get(&(current.x, current.y)) {
            if dist < path.len() { continue; }
        }

        for (&direction, &vector) in bfsdir {
            let position = Point { x: current.x + vector.x, y: current.y + vector.y };
            if input[&'X'] == position { continue; }
            if let Some(_) = input.values().find(|&&button| button == position) {
                let new_path = path.clone() + &direction.to_string();
                if distances.get(&(position.x, position.y)).map_or(true, |&dist| dist >= new_path.len()) {
                    queue.push((position, new_path.clone()));
                    distances.insert((position.x, position.y), new_path.len());
                }
            }
        }
    }
    all_paths.sort_by_key(|a| a.len());
    all_paths
}

fn get_key_presses(
    input: &HashMap<char, Point>, 
    bfsdir: &HashMap<char, Point>, 
    code: &str, 
    robot: isize, 
    memo: &mut HashMap<String, isize>, 
    dirpad: &HashMap<char, Point>
) -> isize {
    let key = format!("{},{}", code, robot);
    if let Some(&result) = memo.get(&key) { return result; }
    let mut current = 'A';
    let mut length = 0;
    for c in code.chars() {
        let moves = get_command(input, bfsdir, current, c);
        if robot == 0 {
            length += moves[0].len() as isize;
        } else {
            length += moves
                .iter()
                .map(|move_| get_key_presses(&dirpad, bfsdir, move_, robot - 1, memo, dirpad))
                .min()
                .unwrap();
        }
        current = c;
    }
    memo.insert(key, length);
    length
}
