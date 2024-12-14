#![allow(unused_variables)]
#![allow(non_snake_case)]
use Day13::machine::Machine;

fn solve (a: (f64, f64), b: (f64, f64), p: (f64, f64)) -> (f64, f64) {
    let y = (p.1 * a.0 - p.0 * a.1) / (a.0 * b.1 - a.1 * b.0);
    let x = (p.0 - b.0 * y) / a.0;
    (x, y)
}

fn main() {
    let mut machines: Vec<Machine> = std::fs::read_to_string("input.txt").unwrap().trim()
        .split("\n\n").map(|s| Machine::from_string(s.trim())).collect();

    let sum: i64 = machines.iter_mut().map(|m| {
        let d = (m.price_x + 10000000000000, m.price_y + 10000000000000);
        let x = solve((m.a.x as f64, m.a.y as f64), (m.b.x as f64, m.b.y as f64), (d.0 as f64, d.1 as f64));
        if x.0.floor() != x.0 || x.1.floor() != x.1 { return 0 } 
        x.0 as i64 * Machine::COST_A + x.1 as i64 * Machine::COST_B
    }).sum();

    println!("{}", sum);
}