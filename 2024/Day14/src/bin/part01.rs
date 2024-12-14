use Day14::robot::{Robot, make_grid};
// clear && cargo run --release --bin part01

fn sum_grid(grid: &Vec<Vec<i64>>, from: (i64, i64), size: (i64, i64)) -> i64 {
    let mut sum = 0;
    for y in from.1..(from.1 + size.1) {
        for x in from.0..(from.0 + size.0) {
            sum += grid[y as usize][x as usize];
        }
    }
    sum
}

fn main() {
    let mut robots: Vec<Robot> = std::fs::read_to_string("input.txt").unwrap()
        .trim().lines().map(|s| Robot::from_string(s.trim())).collect();

    let size: (i64, i64) = (101, 103);
    let half = (size.0 / 2, size.1 / 2);
    let steps = 100; 

    for r in robots.iter_mut() {
        r.update(size, steps);
    }
    
    let grid = make_grid(&robots, size);
    let safety_factor = 
        sum_grid(&grid, (0, 0), half) *
        sum_grid(&grid, (half.0 + 1, half.1 + 1), half) *
        sum_grid(&grid, (0, half.1 + 1), half) *
        sum_grid(&grid, (half.0 + 1, 0), half);

    println!("{:?}", safety_factor);
}
