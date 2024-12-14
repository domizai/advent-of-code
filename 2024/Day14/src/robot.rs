use regex::Regex;

#[derive(Debug, Clone)]
pub struct Robot {
    pub x: i64,
    pub y: i64,
    pub dx: i64,
    pub dy: i64,
} 

impl Robot {
    pub fn from_string(s: &str) -> Robot {
        let re = Regex::new(r"=(?<x>\d+),(?<y>\d+) v=(?<dx>-?\d+),(?<dy>-?\d+)").unwrap();
        let cap = re.captures(s).unwrap();
        let x = cap.name("x").unwrap().as_str().parse::<i64>().unwrap();
        let y = cap.name("y").unwrap().as_str().parse::<i64>().unwrap();
        let dx = cap.name("dx").unwrap().as_str().parse::<i64>().unwrap();
        let dy = cap.name("dy").unwrap().as_str().parse::<i64>().unwrap();
        Robot { x, y, dx, dy }
    }

    pub fn update(&mut self, size: (i64, i64), steps: i64) {
        for _ in 0..steps {
            self.x = (self.x + self.dx).rem_euclid(size.0);
            self.y = (self.y + self.dy).rem_euclid(size.1);
        }
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
