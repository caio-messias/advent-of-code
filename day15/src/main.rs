use std::fs;

use intcode::IntcodeMachine;
use std::collections::{HashMap, VecDeque, HashSet};

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
    Space,
    Oxygen,
    Wall,
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
    fn to(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

fn backtrack(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        Direction::East => Direction::West,
    }
}

/// Returns the position the robot will have if moved in a given direction from the current position.
fn walk_position(from: Direction, position: Position) -> Position {
    match from {
        Direction::North => Position { x: position.x, y: position.y + 1 },
        Direction::East => Position { x: position.x + 1, y: position.y },
        Direction::South => Position { x: position.x, y: position.y - 1 },
        Direction::West => Position { x: position.x - 1, y: position.y },
    }
}

/// Tries to walk the robot in a given direction. If the position in that direction is unvisited and
/// is not a wall, the robot is moved there and the function returns true. Otherwise, false.
///
/// Whenever the robot is moved the function does the necessary bookkeeping to continue exploring the
/// maze with the robot, such as updating the current position, visited tiles and which steps
/// where taken to get to the current position, to enable backtracking.
fn walk(
    machine: &mut IntcodeMachine,
    maze: &mut HashMap<Position, Tile>,
    footsteps: &mut Vec<Direction>,
    direction: Direction,
    current_position: &mut Position,
) -> bool {
    let next_position = walk_position(direction, *current_position);
    if !maze.contains_key(&next_position) {
        machine.add_input(direction.to());
        let out = machine.run()[0];

        match out {
            0 => maze.insert(next_position, Tile::Wall),
            1 => maze.insert(next_position, Tile::Space),
            2 => maze.insert(next_position, Tile::Oxygen),
            _ => panic!("unknown repair droid status: {}", out),
        };

        if out == 1 || out == 2 {
            footsteps.push(direction);
            *current_position = next_position;
            return true;
        }
    }

    return false;
}

fn bfs(
    maze: &HashMap<Position, Tile>,
    initial_position: Position,
) -> HashMap<Position, u32> {
    let mut to_visit = VecDeque::<Position>::new();
    let mut visited = HashSet::<Position>::new();
    let mut distance = HashMap::<Position, u32>::new();

    to_visit.push_back(initial_position);
    visited.insert(initial_position);
    distance.insert(initial_position, 0);

    while let Some(node) = to_visit.pop_front() {
        for child in get_adjacent_nodes(maze, node).iter() {
            // If the adjacent node is not a wall:
            if let Some(child) = child {
                if !visited.contains(child) {
                    let level = distance.get(&node).unwrap_or(&0) + 1;

                    visited.insert(*child);
                    to_visit.push_back(*child);
                    distance.insert(*child, level);
                }
            }
        }
    }

    return distance;
}

fn get_adjacent_nodes(maze: &HashMap<Position, Tile>, position: Position) -> [Option<Position>; 4] {
    let mut nodes: [Option<Position>; 4] = [None; 4];

    let north_position = Position { x: position.x, y: position.y + 1 };
    let tile = maze.get(&north_position).unwrap_or(&Tile::Wall);
    if tile == &Tile::Space || tile == &Tile::Oxygen {
        nodes[0] = Some(north_position);
    }

    let west_position = Position { x: position.x - 1, y: position.y };
    let tile = maze.get(&west_position).unwrap_or(&Tile::Wall);
    if tile == &Tile::Space || tile == &Tile::Oxygen {
        nodes[1] = Some(west_position);
    }

    let east_position = Position { x: position.x + 1, y: position.y };
    let tile = maze.get(&east_position).unwrap_or(&Tile::Wall);
    if tile == &Tile::Space || tile == &Tile::Oxygen {
        nodes[2] = Some(east_position);
    }

    let south_position = Position { x: position.x, y: position.y - 1 };
    let tile = maze.get(&south_position).unwrap_or(&Tile::Wall);
    if tile == &Tile::Space || tile == &Tile::Oxygen {
        nodes[3] = Some(south_position);
    }

    return nodes;
}

fn build_maze(mut machine: IntcodeMachine) -> HashMap<Position, Tile> {
    let mut maze: HashMap<Position, Tile> = HashMap::new();
    let mut footsteps = vec![];

    let mut current_pos = Position { x: 0, y: 0 };
    maze.insert(current_pos, Tile::Space);

    loop {
        if walk(&mut machine, &mut maze, &mut footsteps, Direction::North, &mut current_pos) {
            continue;
        }

        if walk(&mut machine, &mut maze, &mut footsteps, Direction::West, &mut current_pos) {
            continue;
        }

        if walk(&mut machine, &mut maze, &mut footsteps, Direction::East, &mut current_pos) {
            continue;
        }

        if walk(&mut machine, &mut maze, &mut footsteps, Direction::South, &mut current_pos) {
            continue;
        }

        // No new spaces to explore in the current position, backtrack until a position with spaces
        // to explore is found. If the robot returns to the initial position (ie. footsteps is empty),
        // then the entire maze is explored.
        if let Some(step) = footsteps.pop() {
            let previous_direction = backtrack(step);
            machine.add_input(previous_direction.to());
            machine.run();
            current_pos = walk_position(previous_direction, current_pos);
        } else {
            break;
        }
    }

    return maze;
}

fn main() {
    let tape = read_input();
    let machine = IntcodeMachine::new(tape.clone());

    let maze = build_maze(machine);
    let (oxygen_position, _) = maze.iter().find(|(_position, &tile)| tile == Tile::Oxygen).unwrap();

    let levels = bfs(&maze, Position { x: 0, y: 0 });
    println!("Part 1: {:?}", levels.get(oxygen_position).unwrap());

    let levels = bfs(&maze, *oxygen_position);
    println!("Part 2: {}", levels.values().max().unwrap());
}
