use itertools::Itertools;
use intcode::IntcodeMachine;
use std::fs;
use std::cmp::max;

fn read_input() -> Vec<isize> {
    return fs::read_to_string("input")
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<isize>().unwrap())
        .collect();
}

fn max_signal(tape: &Vec<isize>) -> i32 {
    let mut largest_output_signal = 0;
    let phases = (0..5).permutations(5);

    for phase in phases {
        let mut amp_a = IntcodeMachine::new(tape.clone())
            .with_input(phase[0])
            .with_input(0);
        amp_a.run();

        let mut amp_b = IntcodeMachine::new(tape.clone())
            .with_input(phase[1])
            .with_input(amp_a.output.unwrap());
        amp_b.run();

        let mut amp_c = IntcodeMachine::new(tape.clone())
            .with_input(phase[2])
            .with_input(amp_b.output.unwrap());
        amp_c.run();

        let mut amp_d = IntcodeMachine::new(tape.clone())
            .with_input(phase[3])
            .with_input(amp_c.output.unwrap());
        amp_d.run();

        let mut amp_e = IntcodeMachine::new(tape.clone())
            .with_input(phase[4])
            .with_input(amp_d.output.unwrap());
        amp_e.run();

        largest_output_signal = max(largest_output_signal, amp_e.output.unwrap());
    }

    return largest_output_signal as i32
}

fn max_signal_with_feedback(tape: &Vec<isize>) -> i32 {
    let mut largest_output_signal = 0;
    let phases = (5..10).permutations(5);

    for phase in phases {
        let mut amp_a = IntcodeMachine::new(tape.clone()).with_input(phase[0]);
        let mut amp_b = IntcodeMachine::new(tape.clone()).with_input(phase[1]);
        let mut amp_c = IntcodeMachine::new(tape.clone()).with_input(phase[2]);
        let mut amp_d = IntcodeMachine::new(tape.clone()).with_input(phase[3]);
        let mut amp_e = IntcodeMachine::new(tape.clone()).with_input(phase[4]);

        let mut output_end = 0;
        loop {
            amp_a.add_input(output_end);
            amp_a.run();

            amp_b.add_input(amp_a.output.unwrap());
            amp_b.run();

            amp_c.add_input(amp_b.output.unwrap());
            amp_c.run();

            amp_d.add_input(amp_c.output.unwrap());
            amp_d.run();

            amp_e.add_input(amp_d.output.unwrap());
            amp_e.run();
            output_end = amp_e.output.unwrap();

            largest_output_signal = max(largest_output_signal, output_end);
            if amp_e.is_halted() {
                break;
            }
        }
    }

    return largest_output_signal as i32
}

fn main() {
    let tape = read_input();

    println!("Part 1: {}", max_signal(&tape));
    println!("Part 2: {}", max_signal_with_feedback(&tape))
}
