// rustc parts.rs -Oo main && ./main
// to enable dev_only! macro:
// rustc parts.rs -o main && ./main

#![allow(dead_code, unused_variables)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

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

const COST_STEP: isize = 1;
const COST_TURN: isize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
struct Dir (isize, isize);
impl Dir {
    pub fn rotate(&self, a: isize) -> Dir {
        let a = a.signum();
        match a {
            0 => *self,
            _ => Dir (self.1 * a, self.0 * -a),
        }
    }
}

impl From<Dir> for usize {
    fn from(dir: Dir) -> usize {
        match Dir (dir.0.signum(), dir.1.signum()) {
            Dir (-1, 0) | Dir (1, 0) => 0,
            Dir (0, -1) | Dir (0, 1) => 1,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos (usize, usize);
impl Add<Dir> for Pos {
    type Output = Pos;
    fn add(self, dir: Dir) -> Pos {
        Pos ((self.0 as isize + dir.0) as usize, (self.1 as isize + dir.1) as usize)
    }
}

fn solve(
    maze: &Vec<Vec<char>>, 
    start: (Pos, Dir), 
    end: &Pos
) -> HashMap<(Pos, usize), (HashSet<(Pos, usize)>, isize)> {

    // (position, direction) -> (parents, cost)
    let mut map: HashMap<(Pos, usize), (HashSet<(Pos, usize)>, isize)> = HashMap::new();
    // (position, direction, cost, parent)
    let mut queue: VecDeque<(Pos, Dir, isize, Option<(Pos, usize)>)> = VecDeque::new();
    queue.push_back((start.0, start.1.into(), 0, None));

    while let Some((pos, dir, cost, parent)) = queue.pop_front() {
        if maze[pos.1][pos.0] == '#' { continue; }
        let d: usize = dir.into();

        // check if the node has been visited before
        match map.get_mut(&(pos, d)) {
            // update if cost is lower and replace parent
            Some((parents, existing_cost)) if cost < *existing_cost => {
                *existing_cost = cost;
                *parents = parent.into_iter().collect(); 
            }
            Some((parents, existing_cost)) => {
                // add node to existing parent if cost is the same
                if cost == *existing_cost {
                    if let Some(p) = parent { parents.insert(p); }
                }
                continue;
            }
            // node has not been visited, insert it into the map
            None => {
                let parent = parent.into_iter().collect();
                map.insert((pos, d), (parent, cost));
            }
        }
        // if we reached the end
        if *end == pos { continue; }
        // otherwise explore the neighbors
        [0, -1, 1].iter().for_each(|&a| {
            let next_dir = dir.rotate(a);
            let next_pos = pos + next_dir;
            let next_cost = cost + COST_STEP + a.abs() * COST_TURN;
            queue.push_back((next_pos, next_dir, next_cost, Some((pos, d))));
        });
    }
    map
}

fn backtrace(
    map: &HashMap<(Pos, usize), (HashSet<(Pos, usize)>, isize)>, 
    start_node: (Pos, usize), 
) -> HashSet<Pos> {

    let mut path: HashSet<Pos> = HashSet::new();
    let mut stack = vec![start_node];

    while let Some(node) = stack.pop() {
        path.insert(node.0);

        if let Some((parents, _)) = map.get(&node) {
            for &parent in parents {
                if !path.contains(&parent.0) { 
                    stack.push(parent);
                 }
            }
        }
    }
    path
}

fn print_maze(maze: &Vec<Vec<char>>, path: &HashSet<Pos>) {
    let mut maze: Vec<Vec<String>> = maze.iter().map(|row| row.iter().map(|c| {
        if *c == '.' { format!("\x1B[1;30m{}\x1B[0m", c) } else { c.to_string() }
    }).collect()).collect();
    
    for pos in path {
        maze[pos.1][pos.0] = format!("\x1B[1;31m{}\x1B[0m", 'o');
    }

    maze.iter().for_each(|row| println!("{}", row.join("")));
}

fn main() {
    let input = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    let input = std::fs::read_to_string("input.txt").unwrap();
    let maze: Vec<Vec<char>> = input.trim().lines().map(|s| s.chars().collect()).collect();
    let size = Pos (maze[0].len(), maze.len());
    let end = Pos (size.0 - 2, 1);
    let start = (Pos (1, size.1 - 2), Dir (1, 0));

    // solve the maze
    let map = solve(&maze, start, &end);

    // find the direction with the lowest cost at the end
    let dir: usize = match (map.get(&(end, 0)), map.get(&(end, 1))) {
        (Some((_, cost1)), Some((_, cost2))) => if cost1 < cost2 { 0 } else { 1 },
        (Some(_), None) => 0,
        (None, Some(_)) => 1,
        _ => panic!("No path found"),
    };

    // part 1
    println!("lowest score: {:?}", map.get(&(end, dir)).unwrap().1);
    
    // part 2
    let path = backtrace(&map, (end, dir));
    dev_only! { print_maze(&maze, &path); }
    println!("{:?}", path.len());
}
