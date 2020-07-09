use std::fs;
use std::cmp::{max, min};

fn read_input(path: &str) -> (Vec<String>, Vec<String>) {
    let input: String = fs::read_to_string(path)
        .expect("Failed to read input file.");

    let mut lines = input.lines();

    let wire1: Vec<String> = lines.next().unwrap().split(",").map(|x: &str| x.to_string()).collect();
    let wire2: Vec<String> = lines.next().unwrap().split(",").map(|x: &str| x.to_string()).collect();

    return (wire1, wire2);
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self) -> i32 {
        // Calculate manhattan distance from origin
        return self.x.abs() + self.y.abs();
    }
}

fn move_one(point: &Point, direction: char, length: i32) -> Point {
    match direction {
        'R' => Point {
            x: point.x + length,
            y: point.y,
        },
        'L' => Point {
            x: point.x - length,
            y: point.y,
        },
        'U' => Point {
            x: point.x,
            y: point.y + length,
        },
        'D' => Point {
            x: point.x,
            y: point.y - length,
        },
        _ => panic!("Unknown direction"),
    }
}

#[derive(Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn length(&self) -> i32 {
        return (self.end.x - self.start.x).abs() + (self.end.y - self.start.y).abs();
    }

    fn distance_from_start(&self, point: &Point) -> i32 {
        return (point.x - self.start.x).abs() + (point.y - self.start.y).abs();
    }

    fn is_vertical(&self) -> bool {
        return self.start.x == self.end.x;
    }

    fn is_horizontal(&self) -> bool {
        return self.start.y == self.end.y;
    }

    fn same_direction(&self, other: &Line) -> bool {
        return self.is_vertical() && other.is_vertical() || self.is_horizontal() && other.is_horizontal();
    }
}

fn intersection(line1: &Line, line2: &Line) -> Option<Point> {
    if line1.same_direction(&line2) {
        return None;
    }

    let (vline, hline) = if line1.is_vertical() { (line1, line2) } else { (line2, line1) };

    let horizontal_range = min(hline.start.x, hline.end.x)..=max(hline.start.x, hline.end.x);
    let vertical_range = min(vline.start.y, vline.end.y)..=max(vline.start.y, vline.end.y);

    if horizontal_range.contains(&vline.start.x) && vertical_range.contains(&hline.start.y) {
        return Some(Point { x: vline.start.x, y: hline.start.y });
    }

    return None;
}

fn parse_lines(lines: Vec<String>) -> Vec<Line> {
    let mut last_pos = Point { x: 0, y: 0 };

    return lines.iter()
        .map(|s: &String| {
            let mut chars = s.chars();
            let direction = chars.next().unwrap();
            let length = chars.collect::<String>().parse::<i32>().unwrap();

            let new_point = move_one(&last_pos, direction, length);
            let new_line = Line { start: last_pos, end: new_point };
            last_pos = new_point;

            new_line
        })
        .collect();
}

fn shortest_manhattan_and_steps(wire1: &Vec<Line>, wire2: &Vec<Line>) -> (i32, i32) {
    let mut min_manhattan = std::i32::MAX;
    let mut min_distance = std::i32::MAX;

    let mut w1_len = 0;
    let mut w2_len = 0;

    for line1 in wire1 {
        for line2 in wire2 {
            match intersection(line1, line2) {
                Some(point) => {
                    min_manhattan = min(min_manhattan, point.manhattan_distance());

                    let d1 = line1.distance_from_start(&point);
                    let d2 = line2.distance_from_start(&point);
                    min_distance = min(min_distance, w1_len + d1 + w2_len + d2);
                }
                _ => {}
            }
            w2_len += line2.length();
        }
        w2_len = 0;
        w1_len += line1.length();
    }

    return (min_manhattan, min_distance);
}

fn main() {
    let (wire1, wire2) = read_input("input");
    let wire1: Vec<Line> = parse_lines(wire1);
    let wire2: Vec<Line> = parse_lines(wire2);

    let (min_manhattan, min_distance) = shortest_manhattan_and_steps(&wire1, &wire2);

    println!("Part 1: {}", min_manhattan);
    println!("Part 2: {}", min_distance);
}
