use Day14::robot::{Robot, make_grid, print_grid};

fn floodfill(grid: &mut Vec<Vec<i64>>, x: i64, y: i64, density: &mut i64) {
    if x < 0 || y < 0 || x >= grid[0].len() as i64 || y >= grid.len() as i64 { return; }
    if grid[y as usize][x as usize] == 0 { return; }
    *density += grid[y as usize][x as usize]; 
    grid[y as usize][x as usize] = 0;
    floodfill(grid, x + 1, y, density);
    floodfill(grid, x - 1, y, density);
    floodfill(grid, x, y + 1, density);
    floodfill(grid, x, y - 1, density);
}

fn main() {
    let robots_orig: Vec<Robot> = std::fs::read_to_string("input.txt").unwrap()
        .trim().lines().map(|s| Robot::from_string(s.trim())).collect();

    let mut robots = robots_orig.clone();
    let size: (i64, i64) = (101, 103);
    let mut max_density_overall_step: (i64, i64) = (0, 0); // (step, density)

    // find maximum density over a period of time
    for i in 0.. {
        let mut grid = make_grid(&robots, size);
        let mut max_density_step: i64 = 0;
        // find the maximum density of the current frame
        for y in 0..size.1 {
            for x in 0..size.0 {
                let mut density = 0;
                floodfill(&mut grid, x, y, &mut density);
                max_density_step = max_density_step.max(density);
            }
        }
        // update maxium density
        if max_density_step > max_density_overall_step.1 {
            max_density_overall_step = (i, max_density_step);
        }
        // exit if the density is decreasing
        if max_density_overall_step.1 - max_density_step > 30  {
            break;
        }
        // update the robots
        for r in robots.iter_mut() {
            r.update(size, 1);
        }
    }
    // print the grid at the maximum density
    let mut robots = robots_orig.clone();
    for r in robots.iter_mut() {
        r.update(size, max_density_overall_step.0);
    }
    let grid = make_grid(&robots, size);
    print_grid(&grid);
    println!("step: {:?}, density: {:?}", max_density_overall_step.0, max_density_overall_step.1);
}
