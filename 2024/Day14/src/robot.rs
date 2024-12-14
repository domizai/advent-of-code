use regex::{Captures, Regex};

#[derive(Debug, Clone)]
pub struct Robot {
    pub x: i64,
    pub y: i64,
    pub dx: i64,
    pub dy: i64,
} 

impl Robot {
    pub fn from_string(s: &str) -> Robot {
        let cap = Regex::new(r"=(?<x>\d+),(?<y>\d+) v=(?<dx>-?\d+),(?<dy>-?\d+)").unwrap().captures(s).unwrap();
        let x = Self::parse(&cap, "x");
        let y = Self::parse(&cap, "y");
        let dx = Self::parse(&cap, "dx");
        let dy = Self::parse(&cap, "dy");
        Robot { x, y, dx, dy }
    }

    pub fn update(&mut self, size: (i64, i64), steps: i64) {
        self.x = (self.x + self.dx * steps).rem_euclid(size.0);
        self.y = (self.y + self.dy * steps).rem_euclid(size.1);
    }

    fn parse(cap: &Captures<'_>, s: &str) -> i64 {
        cap.name(s).unwrap().as_str().parse::<i64>().unwrap()
    }
}

pub fn make_grid(robots: &Vec<Robot>, size: (i64, i64)) -> Vec<Vec<i64>> {
    let mut grid = vec![vec![0; size.0 as usize]; size.1 as usize];
    for r in robots.iter() {
        grid[r.y as usize][r.x as usize] += 1;
    }
    grid
}

pub fn print_grid(grid: &Vec<Vec<i64>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{:?}", cell);
        }
        println!();
    }
}
