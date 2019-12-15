use std::collections::{HashMap, VecDeque, HashSet};
use std::io::{BufReader, BufRead};
use std::fs::File;

fn read_input(path: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let lines = BufReader::new(File::open(path).unwrap()).lines();

    for line in lines {
        let line = line.unwrap();
        let mut iter = line.split(")");

        let parent = iter.next().unwrap().to_string();
        let child = iter.next().unwrap().to_string();

        graph.entry(parent.clone()).or_insert(Vec::new())
            .push(child.clone());

        // Make it undirected for part 2
        graph.entry(child.clone()).or_insert(Vec::new())
            .push(parent.clone());
    }

    return graph;
}

fn num_orbits(graph: &HashMap<String, Vec<String>>, start: String) -> i32 {
    let mut num_orbits = 0;
    let mut to_visit = VecDeque::<&String>::new();
    let mut visited = HashSet::<&String>::new();
    let mut levels = HashMap::<&String, i32>::new();

    to_visit.push_back(&start);
    visited.insert(&start);
    levels.insert(&start, 0);

    while let Some(node) = to_visit.pop_front() {
        if let Some(children) = graph.get(node) {
            for child in children {
                if !visited.contains(child) {
                    let level = levels.get(node).unwrap() + 1;
                    num_orbits += level;

                    to_visit.push_back(child);
                    visited.insert(child);
                    levels.insert(child, level);
                }
            }
        }
    }

    return num_orbits;
}

fn shortest_distance(graph: &HashMap<String, Vec<String>>, start: String, end: String) -> Option<i32> {
    let mut to_visit = VecDeque::<&String>::new();
    let mut visited = HashSet::<&String>::new();
    let mut levels = HashMap::<&String, i32>::new();

    to_visit.push_back(&start);
    visited.insert(&start);
    levels.insert(&start, 0);

    while let Some(node) = to_visit.pop_front() {
        if let Some(children) = graph.get(node) {
            for child in children {
                if !visited.contains(child) {
                    let level = levels.get(node).unwrap() + 1;

                    if *child == end {
                        return Some(level - 2); // Do not count the start and end connections to their parents
                    }

                    visited.insert(child);
                    to_visit.push_back(child);
                    levels.insert(child, level);
                }
            }
        }
    }

    return None;
}

fn main() {
    let graph = read_input("input");
    println!("Part 1: {}", num_orbits(&graph, "COM".to_string()));
    println!("Part 2: {}", shortest_distance(&graph, "YOU".to_string(), "SAN".to_string()).unwrap());
}
