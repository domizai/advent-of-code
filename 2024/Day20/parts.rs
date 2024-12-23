// rustc parts.rs -Oo main && ./main

use std::collections::{HashSet, HashMap, VecDeque};
use std::ops::Add;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos (pub usize, pub usize);

impl Add<(isize, isize)> for Pos {
    type Output = Pos;
    fn add(self, pos: (isize, isize)) -> Pos {
        Pos ((self.0 as isize + pos.0) as usize, (self.1 as isize + pos.1) as usize)
    }
}

fn find(track: &Vec<Vec<char>>, ch: char) -> Vec<Pos> {
    track.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &c)| {
            if c == ch { Some(Pos (x, y)) } else { None }
        })
    }).collect()
}

fn taxicab(centre: &Pos, dist: usize) -> HashSet<Pos> {
    let mut contour: HashSet<Pos> = HashSet::new();
    for x in 0..=dist as isize {
        let y = dist as isize - x;
        contour.insert(*centre + (x, y));
        contour.insert(*centre + (x, -y));
        contour.insert(*centre + (-x, y));
        contour.insert(*centre + (-x, -y));
    }
    contour
}

fn flood(map: &Vec<Vec<char>>, start: &Pos, end: &Pos) -> HashMap<Pos, usize> {
    let mut path: HashMap<Pos, usize> = HashMap::new();
    let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
    queue.push_back((*start, 0));

    while let Some((pos, cost)) = queue.pop_front() {
        if pos.0 >= map[0].len() || pos.1 >= map.len() { continue; }
        if map[pos.1][pos.0] == '#' { continue; }
        if path.contains_key(&pos) { continue; }
        path.insert(pos, cost);
        if *end == pos { break; }

        [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().for_each(|&p| {
            queue.push_back((pos + p, cost + 1));
        });
    }
    path
}

fn cheat(track: &HashMap<Pos, usize>, picos: isize, min_save: isize) -> usize {
    let mut sum = 0;
    for (pos, cost) in track.iter() {
        for skip in 2..=picos {
            let pos: HashSet<Pos> = taxicab(&pos, skip as usize)
                .iter()
                .filter(|&&n| track.contains_key(&n) && track.get(&n).unwrap() > cost)
                .cloned()
                .collect();

            for p in pos.iter() {
                let c = track.get(&p).unwrap();
                if c <= cost || (c - *cost) as isize - skip < min_save { continue; }
                sum += 1; 
            }
        }
    }
    sum
}

fn main() {
    let map: Vec<Vec<char>> = read_to_string("input.txt").unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let start: Pos = find(&map, 'S')[0];
    let end: Pos = find(&map, 'E')[0];
    let track = flood(&map, &start, &end);
    
    println!("{:?}", cheat(&track, 2, 100));
    println!("{:?}", cheat(&track, 20, 100));
}
