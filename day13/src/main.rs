use std::cmp::Ordering;
use std::fs;

use intcode::IntcodeMachine;

fn read_input() -> Vec<i64> {
    return fs::read_to_string("input")
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Tile {
    fn from(value: i64) -> Tile {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unknown tile code"),
        }
    }
}

fn compare_position(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn main() {
    let mut tape = read_input();

    // Part 1
    let mut machine = IntcodeMachine::new(tape.clone());
    let output = machine.run();

    let block_tiles = output.chunks(3)
        .filter(|&output| Tile::from(output[2]) == Tile::Block)
        .count();

    println!("Part 1: {}", block_tiles);

    // Part 2
    // Memory address 0 represents the number of quarters that have been inserted;
    // set it to 2 to play for free.
    tape[0] = 2;

    let mut machine = IntcodeMachine::new(tape);
    let mut score = 0;
    let mut ball_x = 0;
    let mut pallet_x = 0;

    while !machine.halted() {
        let output = machine.run();

        for output in output.chunks(3) {
            let x = output[0];
            let y = output[1];

            if x == -1 && y == 0 {
                score = output[2];
            } else {
                match Tile::from(output[2]) {
                    Tile::Ball => ball_x = x,
                    Tile::Paddle => pallet_x = x,
                    _ => {}
                }
            }
        }

        machine.add_input(compare_position(ball_x, pallet_x))
    }

    println!("Part 2: {}", score);
}
