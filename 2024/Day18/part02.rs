// rustc parts.rs -Oo main && ./main
// to enable dev_only! macro and show process:
// rustc parts.rs -o main && ./main

#![allow(dead_code, unused_variables)]

mod solver;
use solver::{Pos, solve};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

const SIZE: usize = 71;
const BYTES: usize = 1024;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let corrupted: Vec<Pos> = input.trim().lines().map(|s| {
        let pos: Vec<usize> = s.split(",").map(|s| s.parse().unwrap()).collect();
        Pos (pos[0], pos[1])
    }).collect();

    let start = Pos (0, 0);
    let end = Pos (SIZE - 1, SIZE - 1);

    println!("Spawning threads...");
    
    let mut handles: Vec<JoinHandle<Option<(usize, Pos)>>> = Vec::new();
    let counter: Arc<Mutex<usize>> = Arc::new(Mutex::new(BYTES - 1));

    for n in BYTES..corrupted.len() {
        let bytes = corrupted[0..=n].to_vec();
        let counter = counter.clone();
        let len = corrupted.len();

        handles.push(thread::spawn(move || {
            let map = solve(&bytes, start, &end, (SIZE, SIZE));
            
            dev_only! {
                let mut counter = counter.lock().unwrap();
                *counter += 1;
                println!("{}/{} {:.2}%", *counter, len - 1, *counter as f32 / (len - 1) as f32 * 100.0);
                drop(counter); // unlock the mutex
            }

            match map.get(&end) {
                Some(_) => None,
                _ => Some((n, bytes[n]))
            }
        }));
    }

    match handles.into_iter()
        .filter_map(|h| h.join().unwrap())
        .collect::<Vec<(usize, Pos)>>()
        .iter()
        .min_by_key(|(n, _)| *n) {
            Some((n, pos)) => println!("n: {}, pos: {:?}", n, pos),
            None => println!("No solution found")
        } 
}