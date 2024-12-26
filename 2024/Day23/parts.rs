use std::collections::{HashSet, HashMap};

fn main() {
    let networks: Vec<Vec<String>> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .lines()
        .map(|l| l.split('-').map(String::from).collect())
        .collect();

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for pair in &networks {
        let a = &pair[0];
        let b = &pair[1];
        connections.entry(a.clone()).or_insert_with(HashSet::new).insert(b.clone());
        connections.entry(b.clone()).or_insert_with(HashSet::new).insert(a.clone());
    }

    let set = add_to_networks(&networks, &connections);

    println!("{}", set.iter()
        .filter(|x| x.starts_with('t') || x.contains(",t"))
        .count());

    // simulate adding one network at a time
    let mut current_networks = networks.clone();
    while current_networks.len() > 1 {
        let next = add_to_networks(&current_networks, &connections);
        current_networks = next
            .iter()
            .map(|x| x.split(',').map(String::from).collect())
            .collect();
    }

    let mut final_result = current_networks[0].clone();
    final_result.sort();
    println!("{}", final_result.join(","));
}

fn add_to_networks(networks: &[Vec<String>], connections: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut set = HashSet::new();
    
    for computers in networks {
        let candidates: Vec<HashSet<String>> = computers
            .iter()
            .map(|c| { 
                connections.get(c)
                    .cloned()
                    .unwrap_or_else(HashSet::new)
            })
            .collect();

        let result = candidates.into_iter().reduce(|a, b| {
            a.intersection(&b).cloned().collect()
        }).unwrap();

        for x in result {
            let mut combined = computers.clone();
            combined.push(x);
            combined.sort();
            set.insert(combined.join(","));
        }
    }
    set
}
