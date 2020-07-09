use std::fs;
use intcode::IntcodeMachine;
use std::collections::HashMap;

fn read_input() -> Vec<i64> {
    return fs::read_to_string("input")
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Black,
    White,
}

impl Color {
    fn from(value: i64) -> Color {
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unknown color: {}", value),
        }
    }

    fn to(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Debug)]
enum TurnDirection {
    TurnLeft,
    TurnRight,
}

impl TurnDirection {
    fn from(value: i64) -> TurnDirection {
        match value {
            0 => TurnDirection::TurnLeft,
            1 => TurnDirection::TurnRight,
            _ => panic!("Unknown turn direction: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Robot {
    facing: Direction,
    position: Position,
}

fn turn_left(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn turn(turn_direction: TurnDirection, direction: &Direction) -> Direction {
    match turn_direction {
        TurnDirection::TurnLeft => turn_left(direction),
        TurnDirection::TurnRight => turn_right(direction),
    }
}

fn walk(from: &Direction, position: &Position) -> Position {
    match from {
        Direction::Up => Position { x: position.x, y: position.y + 1 },
        Direction::Right => Position { x: position.x + 1, y: position.y },
        Direction::Down => Position { x: position.x, y: position.y - 1 },
        Direction::Left => Position { x: position.x - 1, y: position.y },
    }
}

fn turn_and_walk(turn_direction: TurnDirection, robot: &Robot) -> Robot {
    let facing = turn(turn_direction, &robot.facing);
    let position = walk(&facing, &robot.position);
    return Robot { facing, position };
}

fn paint_hull(tape: Vec<i64>, initial_color: &Color) -> HashMap<Position, Color> {
    let mut machine = IntcodeMachine::new(tape);
    let mut painted_tiles: HashMap<Position, Color> = HashMap::new();

    let mut robot = Robot { facing: Direction::Up, position: Position { x: 0, y: 0 } };
    painted_tiles.insert(Position { x: 0, y: 0 }, *initial_color);
    machine.add_input(Color::to(initial_color));

    while !machine.halted() {
        let output = machine.run();
        let color = Color::from(output[0]);
        let turn_direction = TurnDirection::from(output[1]);

        painted_tiles.insert(robot.position, color);
        robot = turn_and_walk(turn_direction, &robot);

        let input = Color::to(painted_tiles.get(&robot.position).unwrap_or(&Color::Black));
        machine.add_input(input);
    }

    return painted_tiles;
}

fn print_painted_hull(painted_tiles: HashMap<Position, Color>) {
    use std::i32;
    use std::cmp::{min, max};

    // Find the edges of the image
    let (left, right, bottom, top) = painted_tiles.keys()
        .fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |acc, position| {
            (min(acc.0, position.x), max(acc.1, position.x), min(acc.2, position.y), max(acc.3, position.y))
        });

    // The robot y axis is inverted
    for y in (bottom..=top).rev() {
        for x in left..=right {
            if painted_tiles.get(&Position { x, y }).unwrap_or(&Color::Black) == &Color::White {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let tape = read_input();

    // Part 1:
    let painted_tiles = paint_hull(tape.clone(), &Color::Black);
    println!("Part 1: {}", painted_tiles.len());

    // Part 2:
    let painted_tiles = paint_hull(tape.clone(), &Color::White);
    println!("Part 2:");
    print_painted_hull(painted_tiles);
}

#[cfg(test)]
mod tests {
    use super::{Robot, TurnDirection, Direction, Color, Position, turn_and_walk};

    #[test]
    fn a_robot_facing_up_that_turns_right_should_go_right() {
        let robot = Robot { facing: Direction::Up, position: Position { x: 1, y: 2 } };
        let robot = turn_and_walk(TurnDirection::TurnRight, &robot);

        assert_eq!(robot.facing, Direction::Right);
        assert_eq!(robot.position, Position { x: 2, y: 2 });
    }

    #[test]
    fn a_robot_facing_right_that_turns_right_should_go_down() {
        let robot = Robot { facing: Direction::Right, position: Position { x: 1, y: 2 } };
        let robot = turn_and_walk(TurnDirection::TurnRight, &robot);

        assert_eq!(robot.facing, Direction::Down);
        assert_eq!(robot.position, Position { x: 1, y: 1 });
    }

    #[test]
    fn a_robot_facing_down_that_turns_right_should_go_left() {
        let robot = Robot { facing: Direction::Down, position: Position { x: 1, y: 2 } };
        let robot = turn_and_walk(TurnDirection::TurnRight, &robot);

        assert_eq!(robot.facing, Direction::Left);
        assert_eq!(robot.position, Position { x: 0, y: 2 });
    }

    #[test]
    fn a_robot_facing_left_that_turns_right_should_go_up() {
        let robot = Robot { facing: Direction::Left, position: Position { x: 1, y: 2 } };
        let robot = turn_and_walk(TurnDirection::TurnRight, &robot);

        assert_eq!(robot.facing, Direction::Up);
        assert_eq!(robot.position, Position { x: 1, y: 3 });
    }

    #[test]
    fn a_robot_facing_up_that_turns_left_should_go_left() {
        let robot = Robot { facing: Direction::Up, position: Position { x: 1, y: 2 } };
        let robot = turn_and_walk(TurnDirection::TurnLeft, &robot);

        assert_eq!(robot.facing, Direction::Left);
        assert_eq!(robot.position, Position { x: 0, y: 2 });
    }

    #[test]
    fn a_robot_facing_left_that_turns_left_should_go_down() {
        let robot = Robot { facing: Direction::Left, position: Position { x: 1, y: 2 } };
        let robot = turn_and_walk(TurnDirection::TurnLeft, &robot);

        assert_eq!(robot.facing, Direction::Down);
        assert_eq!(robot.position, Position { x: 1, y: 1 });
    }

    #[test]
    fn a_robot_facing_down_that_turns_left_should_go_right() {
        let robot = Robot { facing: Direction::Down, position: Position { x: 1, y: 2 } };
        let robot = turn_and_walk(TurnDirection::TurnLeft, &robot);

        assert_eq!(robot.facing, Direction::Right);
        assert_eq!(robot.position, Position { x: 2, y: 2 });
    }

    #[test]
    fn a_robot_facing_right_that_turns_left_should_go_up() {
        let robot = Robot { facing: Direction::Right, position: Position { x: 1, y: 2 } };
        let robot = turn_and_walk(TurnDirection::TurnLeft, &robot);

        assert_eq!(robot.facing, Direction::Up);
        assert_eq!(robot.position, Position { x: 1, y: 3 });
    }
}
