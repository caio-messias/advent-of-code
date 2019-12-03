use std::fs;

fn read_input(path: &str) -> Vec<usize> {
    return fs::read_to_string(path)
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
}

struct IntcodeMachine {
    tape: Vec<usize>,
    initial_tape: Vec<usize>,
    position: usize,
}

impl IntcodeMachine {
    fn new(tape: Vec<usize>) -> IntcodeMachine {
        IntcodeMachine {tape: tape.clone(), initial_tape: tape, position: 0}
    }

    fn new_init(mut tape: Vec<usize>, noun: usize, verb: usize) -> IntcodeMachine {
        tape[1] = noun;
        tape[2] = verb;

        IntcodeMachine {tape: tape.clone(), initial_tape: tape, position: 0}
    }

    fn reset(&mut self) {
        self.tape = self.initial_tape.clone();
    }

    fn addi(&mut self) {
        let a = self.tape[self.position+1];
        let b = self.tape[self.position+2];
        let dest = self.tape[self.position+3];

        self.tape[dest] = self.tape[a] + self.tape[b];
        self.position += 4;
    }

    fn mult(&mut self) {
        let a = self.tape[self.position+1];
        let b = self.tape[self.position+2];
        let dest = self.tape[self.position+3];

        self.tape[dest] = self.tape[a] * self.tape[b];
        self.position += 4;
    }

    fn run(&mut self) -> usize {
        loop {
            match self.tape[self.position] {
                1 => self.addi(),
                2 => self.mult(),
                99 => return self.tape[0],
                _ => panic!("unkown opcode")
            }
        }
    }

    fn run_with_target(&mut self, target: usize) -> usize {
        loop {
            match self.tape[self.position] {
                1 => self.addi(),
                2 => self.mult(),
                99 => return self.tape[target],
                _ => panic!("unkown opcode")
            }
        }
    }
}

fn main() {
    let tape: Vec<usize> = read_input("input");

    // Part 1
    let mut machine = IntcodeMachine::new_init(tape.clone(), 12, 02);
    println!("{}", machine.run());

    // part 2
    let desired_output = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut machine = IntcodeMachine::new_init(tape.clone(), noun, verb);
            if machine.run() == desired_output {
                println!("100 * {} + {} = {}", noun, verb, 100* noun + verb);
            }
            machine.reset();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::IntcodeMachine;

    #[test]
    fn example1() {
        let tape: Vec<usize> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run(), 3500)
    }

    #[test]
    fn example2() {
        let tape: Vec<usize> = vec![1,0,0,0,99];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run(), 2)
    }

    #[test]
    fn example3() {
        let tape: Vec<usize> = vec![2,3,0,3,99];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run_with_target(3), 6)
    }

    #[test]
    fn example4() {
        let tape: Vec<usize> = vec![2,4,4,5,99,0];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run_with_target(5), 9801)
    }

    #[test]
    fn example5() {
        let tape: Vec<usize> = vec![1,1,1,4,99,5,6,0,99];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run(), 30)
    }
}
