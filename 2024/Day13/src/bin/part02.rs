#![allow(unused_variables)]
#![allow(non_snake_case)]
use Day13::machine::Machine;

fn solve (a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> Option<(i64, i64)> {
    let _a = (a.0 as f64, a.1 as f64);
    let _b = (b.0 as f64, b.1 as f64);
    let _p = (p.0 as f64, p.1 as f64);
    
    let a: f64 = _a.1 / _a.0;
    let mut yB: f64 = _b.1 - _b.0 * a;
    let yT: f64 = _p.1 - _p.0 * a;
    yB = yT / yB;
    let yA = (_p.1 - yB * _b.1) / _a.1;

    let y: (i64, i64) = (yA.round() as i64, yB.round() as i64);
    let pX: i64 = (_a.0 as i64) * y.0 + (_b.0 as i64) * y.1;
    let pY: i64 = (_a.1 as i64) * y.0 + (_b.1 as i64) * y.1;

    if pX != p.0 as i64 || pY != p.1 as i64 {
        return None;
    }
    Some(y)
}

fn main() {
    let mut machines: Vec<Machine> = std::fs::read_to_string("input.txt").unwrap().trim()
        .split("\n\n").map(|s| Machine::from_string(s.trim())).collect();

    let sum: i64 = machines.iter_mut().map(|m| {
        let p = (m.price_x + 10000000000000, m.price_y + 10000000000000);
        match solve((m.a.x, m.a.y), (m.b.x, m.b.y), p) {
            Some((yA, yB)) => yA * Machine::COST_A + yB * Machine::COST_B,
            None => 0,
        }
    }).sum();

    println!("{}", sum);
}