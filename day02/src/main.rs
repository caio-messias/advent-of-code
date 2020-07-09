use std::fs;
use intcode::IntcodeMachine;

fn read_input(path: &str) -> Vec<i64> {
    return fs::read_to_string(path)
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
}

fn main() {
    let tape: Vec<i64> = read_input("input");

    // Part 1
    let mut machine = IntcodeMachine::new(tape.clone())
        .with_init(12, 02);
    println!("Part 1: {}", machine.run_for_target(0));

    // part 2
    let desired_output = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut machine = IntcodeMachine::new(tape.clone())
                .with_init(noun, verb);

            if machine.run_for_target(0) == desired_output {
                println!("Part 2: 100 * {} + {} = {}", noun, verb, 100* noun + verb);
            }
        }
    }
}
