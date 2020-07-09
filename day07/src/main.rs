use itertools::Itertools;
use intcode::IntcodeMachine;
use std::fs;
use std::cmp::max;

fn read_input() -> Vec<i64> {
    return fs::read_to_string("input")
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
}

fn max_signal(tape: &Vec<i64>) -> i64 {
    let mut largest_output_signal = 0;
    let phases = (0..5).permutations(5);

    for phase in phases {
        let mut amp_a = IntcodeMachine::new(tape.clone())
            .with_input(phase[0])
            .with_input(0);

        let mut amp_b = IntcodeMachine::new(tape.clone())
            .with_input(phase[1])
            .with_input(amp_a.run()[0]);

        let mut amp_c = IntcodeMachine::new(tape.clone())
            .with_input(phase[2])
            .with_input(amp_b.run()[0]);

        let mut amp_d = IntcodeMachine::new(tape.clone())
            .with_input(phase[3])
            .with_input(amp_c.run()[0]);

        let mut amp_e = IntcodeMachine::new(tape.clone())
            .with_input(phase[4])
            .with_input(amp_d.run()[0]);

        largest_output_signal = max(largest_output_signal, amp_e.run()[0]);
    }

    return largest_output_signal;
}

fn max_signal_with_feedback(tape: &Vec<i64>) -> i64 {
    let mut largest_output_signal = 0;
    let phases = (5..10).permutations(5);

    for phase in phases {
        let mut amp_a = IntcodeMachine::new(tape.clone()).with_input(phase[0]);
        let mut amp_b = IntcodeMachine::new(tape.clone()).with_input(phase[1]);
        let mut amp_c = IntcodeMachine::new(tape.clone()).with_input(phase[2]);
        let mut amp_d = IntcodeMachine::new(tape.clone()).with_input(phase[3]);
        let mut amp_e = IntcodeMachine::new(tape.clone()).with_input(phase[4]);

        let mut output_end = 0;
        while !amp_e.halted() {
            amp_a.add_input(output_end);
            amp_b.add_input(amp_a.run()[0]);
            amp_c.add_input(amp_b.run()[0]);
            amp_d.add_input(amp_c.run()[0]);
            amp_e.add_input(amp_d.run()[0]);
            output_end = amp_e.run()[0];

            largest_output_signal = max(largest_output_signal, output_end);
        }
    }

    return largest_output_signal;
}

fn main() {
    let tape = read_input();

    println!("Part 1: {}", max_signal(&tape));
    println!("Part 2: {}", max_signal_with_feedback(&tape))
}
