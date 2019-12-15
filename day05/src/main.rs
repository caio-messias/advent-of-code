use std::fs;
use intcode::IntcodeMachine;

fn read_input(path: &str) -> Vec<isize> {
    return fs::read_to_string(path)
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<isize>().unwrap())
        .collect();
}

fn main() {
    let tape: Vec<isize> = read_input("input");

    // Part 1
    let mut machine = IntcodeMachine::new(tape.clone());
    machine.input = vec![1];
    machine.run();
    println!("Part 1: {:?}", machine.output);

    // Part 2
    let mut machine = IntcodeMachine::new(tape);
    machine.input = vec![5];
    machine.run();
    println!("Part 2: {:?}", machine.output);
}