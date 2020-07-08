use intcode::IntcodeMachine;
use std::fs;

fn read_input() -> Vec<i64> {
    return fs::read_to_string("input")
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
}

fn main() {
    let tape = read_input();

    let mut machine = IntcodeMachine::new(tape.clone())
        .with_input(1);

    machine.run();
    println!("Part 1: {}", machine.output[0]);


    let mut machine = IntcodeMachine::new(tape.clone())
        .with_input(2);

    machine.run();
    println!("Part 2: {}", machine.output[0]);

}
