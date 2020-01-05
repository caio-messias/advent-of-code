use std::collections::VecDeque;

pub struct IntcodeMachine {
    tape: Vec<isize>,
    position: usize,
    pub input: VecDeque<isize>,
    pub output: Option<isize>,
    status: MachineStatus,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ParameterMode {
    Positional,
    Immediate,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MachineStatus {
    Run,
    Yield,
    Halt,
}

impl IntcodeMachine {
    pub fn new(tape: Vec<isize>) -> IntcodeMachine {
        IntcodeMachine {tape, position: 0, input: VecDeque::new(), output: Option::None, status: MachineStatus::Run}
    }

    pub fn with_init(mut self, noun: isize, verb: isize) -> IntcodeMachine {
        self.tape[1] = noun;
        self.tape[2] = verb;
        return self;
    }

    pub fn with_input(mut self, input: isize) -> Self {
        self.add_input(input);
        return self;
    }

    pub fn add_input(&mut self, input: isize) {
        self.input.push_back(input);
    }

    fn parse_mode(&self, i: isize) -> ParameterMode {
        match i {
            1 => ParameterMode::Immediate,
            0 | _ => ParameterMode::Positional,
        }
    }

    fn fetch1mode(&mut self) -> ParameterMode {
        let parameter_mode = self.tape[self.position] / 100;
        self.parse_mode(parameter_mode % 10)
    }

    fn fetch2modes(&mut self) -> (ParameterMode, ParameterMode) {
        let mode1 = self.fetch1mode();
        let mode2 = self.tape[self.position] / 1000;

        (self.parse_mode(mode2 % 10), mode1)
    }

    fn fetch3modes(&mut self) -> (ParameterMode, ParameterMode, ParameterMode) {
        let (mode2, mode1) = self.fetch2modes();
        let mode3 = self.tape[self.position] / 10000;

        (self.parse_mode(mode3 % 10), mode2, mode1)
    }

    fn fetch_arg(&mut self, mode: ParameterMode) -> isize {
        self.position += 1;

        match mode {
            ParameterMode::Positional => {
                let pointer = self.tape[self.position] as usize;
                return self.tape[pointer];
            },
            ParameterMode::Immediate => {
                return self.tape[self.position];
            },
        }
    }

    fn fetch_dest(&mut self) -> usize {
        self.fetch_arg(ParameterMode::Immediate) as usize
    }

    fn store(&mut self, dest: usize, value: isize) {
        self.tape[dest] = value;
    }

    fn add(&mut self) {
        let (_mode3, mode2, mode1) = self.fetch3modes();

        let a = self.fetch_arg(mode1);
        let b = self.fetch_arg(mode2);
        let dest = self.fetch_dest();

        let result = a + b;
        self.store(dest, result);
    }

    fn mul(&mut self) {
        let (_mode3, mode2, mode1) = self.fetch3modes();

        let a = self.fetch_arg(mode1);
        let b = self.fetch_arg(mode2);
        let dest = self.fetch_dest();


        let result = a * b;
        self.store(dest, result);
    }

    fn st(&mut self) {
        let dest = self.fetch_dest();
        let input = self.input.pop_front().unwrap();

        self.store(dest, input);
    }

    fn ld(&mut self) {
        let mode = self.fetch1mode();
        let output = self.fetch_arg(mode);

        self.output = Some(output);
        self.status = MachineStatus::Yield;
    }

    fn jnz(&mut self) {
        let (mode2, mode1) = self.fetch2modes();
        let a = self.fetch_arg(mode1);
        let b = self.fetch_arg(mode2) as usize;

        if a != 0 {
            self.position = b -1;
        }
    }

    fn jz(&mut self) {
        let (mode2, mode1) = self.fetch2modes();
        let a = self.fetch_arg(mode1);
        let b = self.fetch_arg(mode2) as usize;

        if a == 0 {
            self.position = b-1;
        }
    }

    fn tlt(&mut self) {
        let (mode2, mode1) = self.fetch2modes();
        let a = self.fetch_arg(mode1);
        let b = self.fetch_arg(mode2);
        let dest = self.fetch_dest();

        let result = if a < b { 1 } else { 0 };
        self.store(dest, result);
    }

    fn teq(&mut self) {
        let (mode2, mode1) = self.fetch2modes();
        let a = self.fetch_arg(mode1);
        let b = self.fetch_arg(mode2);
        let dest = self.fetch_dest();

        let result = if a == b { 1 } else { 0 };
        self.store(dest, result);
    }

    fn halt(&mut self) {
        self.status = MachineStatus::Halt;
    }

    pub fn is_halted(&self) -> bool {
        return self.status == MachineStatus::Halt;
    }

    pub fn run(&mut self) -> isize {
        return self.run_for_target(0);
    }

    pub fn run_for_target(&mut self, target: usize) -> isize {
        self.status = MachineStatus::Run;

        loop {
            let opcode = self.tape[self.position] % 100;
            match opcode {
                1 => self.add(),
                2 => self.mul(),
                3 => self.st(),
                4 => self.ld(),
                5 => self.jnz(),
                6 => self.jz(),
                7 => self.tlt(),
                8 => self.teq(),
                99 => self.halt(),
                _ => panic!("unkown opcode")
            }
            self.position += 1;

            if self.status == MachineStatus::Halt {
                return self.tape[target] as isize;
            }

            if self.status == MachineStatus::Yield {
                return self.output.unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntcodeMachine;

    #[test]
    fn example1() {
        let tape: Vec<isize> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run(), 3500)
    }

    #[test]
    fn example2() {
        let tape: Vec<isize> = vec![1, 0, 0, 0, 99];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run(), 2)
    }

    #[test]
    fn example3() {
        let tape: Vec<isize> = vec![2, 3, 0, 3, 99];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run_for_target(3), 6)
    }

    #[test]
    fn example4() {
        let tape: Vec<isize> = vec![2, 4, 4, 5, 99, 0];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run_for_target(5), 9801)
    }

    #[test]
    fn example5() {
        let tape: Vec<isize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run(), 30)
    }

    #[test]
    fn test_addi_different_modes() {
        let tape: Vec<isize> = vec![1002, 4, 3, 4, 33];
        let mut machine = IntcodeMachine::new(tape);

        assert_eq!(machine.run_for_target(4), 99)
    }

    #[test]
    fn test_input_output() {
        let tape: Vec<isize> = vec![3, 0, 4, 0, 99];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(1234);

        machine.run();
        assert_eq!(machine.output, Some(1234));
    }

    #[test]
    fn test_eq8_position() {
        let tape: Vec<isize> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(8);

        machine.run();
        assert_eq!(machine.output, Some(1));
    }

    #[test]
    fn test_neq8_position() {
        let tape: Vec<isize> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(5);

        machine.run();
        assert_eq!(machine.output, Some(0));
    }

    #[test]
    fn test_lt8_position() {
        let tape: Vec<isize> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(5);

        machine.run();
        assert_eq!(machine.output, Some(1));
    }

    #[test]
    fn test_nlt8_position() {
        let tape: Vec<isize> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(80);

        machine.run();
        assert_eq!(machine.output, Some(0));
    }

    #[test]
    fn test_eq8_immediate() {
        let tape: Vec<isize> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(8);

        machine.run();
        assert_eq!(machine.output, Some(1));
    }

    #[test]
    fn test_neq8_immediate() {
        let tape: Vec<isize> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(9);

        machine.run();
        assert_eq!(machine.output, Some(0));
    }

    #[test]
    fn test_lt8_immediate() {
        let tape: Vec<isize> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(5);

        machine.run();
        assert_eq!(machine.output, Some(1));
    }

    #[test]
    fn test_nlt8_immediate() {
        let tape: Vec<isize> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(9);

        machine.run();
        assert_eq!(machine.output, Some(0));
    }

    #[test]
    fn test_jump0_position() {
        let tape: Vec<isize> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(0);

        machine.run();
        assert_eq!(machine.output, Some(0));
    }

    #[test]
    fn test_jump1_position() {
        let tape: Vec<isize> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(999);

        machine.run();
        assert_eq!(machine.output, Some(1));
    }

    #[test]
    fn test_jump0_immediate() {
        let tape: Vec<isize> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(0);

        machine.run();
        assert_eq!(machine.output, Some(0));
    }

    #[test]
    fn test_jump1_immediate() {
        let tape: Vec<isize> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(999);

        machine.run();
        assert_eq!(machine.output, Some(1));
    }

    #[test]
    fn test_large_lt() {
        let tape: Vec<isize> = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(7);

        machine.run();
        assert_eq!(machine.output, Some(999));
    }

    #[test]
    fn test_large_eq() {
        let tape: Vec<isize> = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(8);

        machine.run();
        assert_eq!(machine.output, Some(1000));
    }

    #[test]
    fn test_large_gt() {
        let tape: Vec<isize> = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        let mut machine = IntcodeMachine::new(tape)
            .with_input(9);

        machine.run();
        assert_eq!(machine.output, Some(1001));
    }
}