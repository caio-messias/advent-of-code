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

    let mut machine = IntcodeMachine::new(tape)
        .with_input(5);

    machine.run();
    println!("Part 2: {}", machine.output.unwrap());
}