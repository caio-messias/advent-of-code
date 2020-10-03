use intcode::IntcodeMachine;
use std::collections::vec_deque::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs;

fn read_input() -> Vec<i64> {
    return fs::read_to_string("input")
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from(ascii: i64) -> Self {
        match ascii {
            94 => Direction::North,
            60 => Direction::West,
            62 => Direction::East,
            118 => Direction::South,
            _ => panic!("unknown robot direction: {}", ascii),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Scaffold,
    Empty,
    Robot(Direction),
}

impl Tile {
    fn from(ascii: i64) -> Self {
        match ascii {
            35 => Tile::Scaffold,
            46 => Tile::Empty,
            94 | 60 | 62 | 118 => Tile::Robot(Direction::from(ascii)),
            _ => panic!("unknown tile code: {}", ascii),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Robot {
    position: Position,
    direction: Direction,
}

fn main() {
    let tape = read_input();
    let mut machine = IntcodeMachine::new(tape.clone());
    let output = machine.run();

    let scaffolds: HashMap<Position, Tile> = build_scaffolds(&output);
    let intersections: Vec<Position> = find_intersections(&scaffolds);
    let sum_aligment = calc_sum_aligment(&intersections);
    println!("part 1: {}", sum_aligment);

    let path = walk_scaffolds(&scaffolds);
    let movement_function = build_movement_function(&path);

    let mut machine = IntcodeMachine::new(tape)
        .with_zeroth(2)
        .with_inputs(&movement_function);

    let mut output = machine.run();
    let total_dust = output.pop().unwrap();
    println!("part 2: {}", total_dust);
}

fn build_scaffolds(input: &Vec<i64>) -> HashMap<Position, Tile> {
    input
        .iter()
        .fold(
            (HashMap::new(), Position { x: 0, y: 0 }),
            |(mut map, current_pos), code| match code {
                35 | 46 | 94 | 60 | 62 | 118 => {
                    map.insert(current_pos, Tile::from(*code));
                    let next_pos = Position {
                        x: current_pos.x + 1,
                        y: current_pos.y,
                    };
                    (map, next_pos)
                }
                10 => {
                    let next_pos = Position {
                        x: 0,
                        y: current_pos.y - 1,
                    };
                    (map, next_pos)
                }
                _ => panic!("Unknown scaffoling value: {}", code),
            },
        )
        .0
}

fn find_intersections(scaffolds: &HashMap<Position, Tile>) -> Vec<Position> {
    scaffolds.iter().fold(Vec::new(), |mut vec, (pos, _value)| {
        if is_intersection(*pos, &scaffolds) {
            vec.push(*pos);
        }

        vec
    })
}

fn is_intersection(p: Position, scaffolds: &HashMap<Position, Tile>) -> bool {
    let current_tile_is_scaffold = scaffolds
        .get(&Position { x: p.x, y: p.y })
        .unwrap_or(&Tile::Empty)
        == &Tile::Scaffold;

    let up_is_scaffold = scaffolds
        .get(&Position { x: p.x, y: p.y + 1 })
        .unwrap_or(&Tile::Empty)
        == &Tile::Scaffold;

    let right_is_scaffold = scaffolds
        .get(&Position { x: p.x + 1, y: p.y })
        .unwrap_or(&Tile::Empty)
        == &Tile::Scaffold;

    let down_is_scaffold = scaffolds
        .get(&Position { x: p.x, y: p.y - 1 })
        .unwrap_or(&Tile::Empty)
        == &Tile::Scaffold;

    let left_is_scaffold = scaffolds
        .get(&Position { x: p.x - 1, y: p.y })
        .unwrap_or(&Tile::Empty)
        == &Tile::Scaffold;

    return current_tile_is_scaffold
        && up_is_scaffold
        && down_is_scaffold
        && left_is_scaffold
        && right_is_scaffold;
}

fn calc_sum_aligment(intersections: &Vec<Position>) -> i32 {
    // Using -pox.y because the y axis was inverted
    // when building the scaffold.
    intersections
        .iter()
        .fold(0, |acc, pos| acc + pos.x * (-pos.y))
}

fn walk_scaffolds(scaffolds: &HashMap<Position, Tile>) -> String {
    let mut robot = find_robot(&scaffolds);
    let mut path = String::new();
    let mut visited: HashSet<Position> = HashSet::new();

    loop {
        if try_walk(
            &mut robot,
            Direction::North,
            &mut path,
            &scaffolds,
            &mut visited,
        ) {
            continue;
        }

        if try_walk(
            &mut robot,
            Direction::East,
            &mut path,
            &scaffolds,
            &mut visited,
        ) {
            continue;
        }

        if try_walk(
            &mut robot,
            Direction::West,
            &mut path,
            &scaffolds,
            &mut visited,
        ) {
            continue;
        }

        if try_walk(
            &mut robot,
            Direction::South,
            &mut path,
            &scaffolds,
            &mut visited,
        ) {
            continue;
        }

        return path;
    }
}

fn find_robot(scaffolds: &HashMap<Position, Tile>) -> Robot {
    let robot = scaffolds
        .iter()
        .find(|(_pos, tile)| match tile {
            Tile::Robot(_) => true,
            _ => false,
        })
        .unwrap();

    let robot_pos: Position = *robot.0;

    let robot_dir: Direction = if let Tile::Robot(direction) = *robot.1 {
        direction
    } else {
        panic!("could not find the robot direction");
    };

    return Robot {
        position: robot_pos,
        direction: robot_dir,
    };
}

fn try_walk(
    robot: &mut Robot,
    new_direction: Direction,
    path: &mut String,
    scaffolds: &HashMap<Position, Tile>,
    visited: &mut HashSet<Position>,
) -> bool {
    if can_walk(robot.position, new_direction, scaffolds, visited) {
        path.push(turn(robot.direction, new_direction));
        path.push(',');

        let mut count = 0;
        let mut current_robot_pos = robot.position;
        while can_walk(current_robot_pos, new_direction, &scaffolds, &visited) {
            visited.insert(current_robot_pos);
            current_robot_pos = get_next_position(current_robot_pos, new_direction);
            count += 1;
        }

        path.push_str(&count.to_string());
        path.push(',');
        robot.direction = new_direction;
        robot.position = current_robot_pos;
        return true;
    } else {
        return false;
    }
}

fn can_walk(
    from_position: Position,
    to_direction: Direction,
    scaffold: &HashMap<Position, Tile>,
    visited: &HashSet<Position>,
) -> bool {
    let next_position = get_next_position(from_position, to_direction);
    let next_tile = scaffold.get(&next_position).unwrap_or(&Tile::Empty);
    let next_tile_is_scaffold = next_tile == &Tile::Scaffold;
    let not_visited = !visited.contains(&next_position);
    let is_intersection = is_intersection(next_position, &scaffold);
    return next_tile_is_scaffold && (not_visited || is_intersection);
}

fn turn(from: Direction, to: Direction) -> char {
    match (from, to) {
        (Direction::North, Direction::West) => 'L',
        (Direction::North, Direction::East) => 'R',
        (Direction::West, Direction::South) => 'L',
        (Direction::West, Direction::North) => 'R',
        (Direction::South, Direction::East) => 'L',
        (Direction::South, Direction::West) => 'R',
        (Direction::East, Direction::North) => 'L',
        (Direction::East, Direction::South) => 'R',
        _ => panic!("invalid robot turning: from {:?} to {:?}", from, to),
    }
}

fn get_next_position(from_position: Position, to_direction: Direction) -> Position {
    match to_direction {
        Direction::North => Position {
            x: from_position.x,
            y: from_position.y + 1,
        },
        Direction::East => Position {
            x: from_position.x + 1,
            y: from_position.y,
        },
        Direction::South => Position {
            x: from_position.x,
            y: from_position.y - 1,
        },
        Direction::West => Position {
            x: from_position.x - 1,
            y: from_position.y,
        },
    }
}

fn build_movement_function(path: &str) -> VecDeque<i64> {
    let (mut a, mut b, mut c) = compress(&path).unwrap();
    let mut main = create_main(&path, &a, &b, &c);
    main.push('\n');
    a.push('\n');
    b.push('\n');
    c.push('\n');
    let continuous = "n\n";

    let mut input: VecDeque<i64> =
        VecDeque::with_capacity(main.len() + a.len() + b.len() + c.len() + continuous.len());
    input.append(&mut string_to_codes(&main));
    input.append(&mut string_to_codes(&a));
    input.append(&mut string_to_codes(&b));
    input.append(&mut string_to_codes(&c));
    input.append(&mut string_to_codes(continuous));
    return input;
}

fn compress(string: &str) -> Option<(String, String, String)> {
    for a in 1..=20 {
        for b in 1..=20 {
            for c in 1..=20 {
                let mut remaining = String::from(string);
                let mut match_a = String::from(&remaining[0..a]);
                remaining = remaining.replace(&match_a, "");
                let mut match_b = String::from(&remaining[0..b]);
                remaining = remaining.replace(&match_b, "");
                let mut match_c = String::from(&remaining[0..c]);
                remaining = remaining.replace(&match_c, "");
                if remaining.len() == 0 {
                    // remove trailling commas
                    match_a.pop();
                    match_b.pop();
                    match_c.pop();
                    return Some((match_a, match_b, match_c));
                }
            }
        }
    }

    return None;
}

fn create_main(path: &str, a: &str, b: &str, c: &str) -> String {
    let mut main = String::from(path);
    main = main.replace(a, "A");
    main = main.replace(b, "B");
    main = main.replace(c, "C");
    main.pop(); // remove trailling comma
    return main;
}

fn string_to_codes(string: &str) -> VecDeque<i64> {
    string.chars().map(|c| c as i64).collect()
}
