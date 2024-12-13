#![allow(unused_variables)]
use Day13::machine::Machine;
// clear && cargo run --release --bin part01

fn find(costs: &mut Vec<i64>, m: &Machine) {
    let max_pushes_a = (m.price_x / m.a.x).max(m.price_y / m.a.y);
    let max_pushes_b = (m.price_x / m.b.x).max(m.price_y / m.b.y);

    for pushes_a in 0..=max_pushes_a {
        for pushes_b in 0..=max_pushes_b {
            // Check if the current combination satisfies the constraints
            if m.a.x * pushes_a + m.b.x * pushes_b == m.price_x &&
               m.a.y * pushes_a + m.b.y * pushes_b == m.price_y {
                costs.push(pushes_a * Machine::COST_A + pushes_b * Machine::COST_B);
            }
            // Exit early if the bounds are exceeded
            if m.a.x * pushes_a > m.price_x || m.a.y * pushes_a > m.price_y ||
               m.b.x * pushes_b > m.price_x || m.b.y * pushes_b > m.price_y {
                break;
            }
        }
    }
}

fn main() {
    let machines: Vec<Machine> = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176"
.split("\n\n").map(|s| Machine::from_string(s.trim())).collect();
    
    let mut machines: Vec<Machine> = std::fs::read_to_string("input.txt").unwrap().trim()
        .split("\n\n").map(|s| Machine::from_string(s.trim())).collect();

    let sum: i64 = machines.iter_mut().map(|m| {
        let mut costs: Vec<i64> = Vec::new(); 
        find(&mut costs, m);
        *costs.iter().min().unwrap_or(&0)
    }).sum();

    println!("{:?}", sum);
}
