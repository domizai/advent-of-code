use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

#[macro_export]
macro_rules! dev_only {
    ($($body:tt)*) => {
        #[cfg(debug_assertions)] {
            $($body)*
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos (pub usize, pub usize);
impl Add<(isize, isize)> for Pos {
    type Output = Pos;
    fn add(self, pos: (isize, isize)) -> Pos {
        Pos ((self.0 as isize + pos.0) as usize, (self.1 as isize + pos.1) as usize)
    }
}

pub fn solve(
    walls: &Vec<Pos>, 
    start: Pos, 
    end: &Pos,
    bounds: (usize, usize)
) -> HashMap<Pos, (HashSet<Pos>, isize)> {

    let mut map: HashMap<Pos, (HashSet<Pos>, isize)> = HashMap::new();
    let mut queue: VecDeque<(Pos, isize, Option<Pos>)> = VecDeque::new();
    queue.push_back((start, 0, None));

    while let Some((pos, cost, parent)) = queue.pop_front() {
        if pos.0 >= bounds.0 || pos.1 >= bounds.1 { continue; }
        if walls.contains(&pos) { continue; }

        // check if the node has been visited before
        match map.get_mut(&pos) {
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
                map.insert(pos, (parent, cost));
            }
        }
        // if we reached the end
        if *end == pos { continue; }
        // otherwise explore the neighbors
        [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().for_each(|&p| {
            queue.push_back((pos + p, cost + 1, Some(pos)));
        });
    }
    map
}

pub fn backtrace(
    map: &HashMap<Pos, (HashSet<Pos>, isize)>, 
    start_node: Pos, 
) -> HashSet<Pos> {

    let mut path: HashSet<Pos> = HashSet::new();
    let mut stack = vec![start_node];

    while let Some(node) = stack.pop() {
        path.insert(node);

        if let Some((parents, _)) = map.get(&node) {
            // we don't care about multiple solutions, just take the first parent
            if let Some(&parent) = parents.iter().next() {
                stack.push(parent);
            }
        }
    }
    path
}
