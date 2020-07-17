use std::{fmt, fs};
use std::cmp::Ordering;

use regex::Regex;

fn read_input(path: &str) -> Vec<Moon> {
    let input: String = fs::read_to_string(path)
        .expect("Failed to read input file.");

    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let capture = re.captures(&line).unwrap();
            let x: i32 = capture[1].parse().unwrap();
            let y: i32 = capture[2].parse().unwrap();
            let z: i32 = capture[3].parse().unwrap();

            Moon::new(x, y, z)
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon { x, y, z, vx: 0, vy: 0, vz: 0 }
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z, self.vx, self.vy, self.vz)
    }
}

fn compare_position(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn apply_gravity(moon: &mut Moon, other_moon: &Moon) {
    moon.vx += compare_position(moon.x, other_moon.x);
    moon.vy += compare_position(moon.y, other_moon.y);
    moon.vz += compare_position(moon.z, other_moon.z);
}

fn apply_velocity(moon: &mut Moon) {
    moon.x += moon.vx;
    moon.y += moon.vy;
    moon.z += moon.vz;
}

struct Universe {
    moons: Vec<Moon>,
}

impl Universe {
    fn new(moons: Vec<Moon>) -> Universe {
        Universe { moons }
    }
}

impl Iterator for Universe {
    type Item = Vec<Moon>;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 1..=self.moons.len() {
            let (left, right) = self.moons.split_at_mut(i);
            let this_moon = left.last_mut().unwrap();
            for other_moon in right {
                apply_gravity(this_moon, other_moon);
                apply_gravity(other_moon, this_moon);
            }
            apply_velocity(this_moon);
        }
        Some(self.moons.clone())
    }
}

fn calc_total_energy(moons: &[Moon]) -> i32 {
    moons.iter()
        .fold(0, |acc, moon| {
            let potential = moon.x.abs() + moon.y.abs() + moon.z.abs();
            let kinetic = moon.vx.abs() + moon.vy.abs() + moon.vz.abs();
            (acc + potential * kinetic)
        })
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp
    }
    a
}

fn find_universe_cycle(universe: Universe) -> (usize, usize, usize) {
    let initial_moons = universe.moons.clone();
    let (mut x, mut y, mut z): (Option<usize>, Option<usize>, Option<usize>) = (None, None, None);

    for (moons, i) in universe.zip(1..) {
        let (x_repeated, y_repeated, z_repeated) = moons.iter().zip(&initial_moons)
            .fold((true, true, true), |acc, (moon, initial)| {
                (acc.0 && moon.x == initial.x && moon.vx == 0,
                 acc.1 && moon.y == initial.y && moon.vy == 0,
                 acc.2 && moon.z == initial.z && moon.vz == 0)
            });

        if x.is_none() && x_repeated {
            x = Some(i);
        }
        if y.is_none() && y_repeated {
            y = Some(i);
        }
        if z.is_none() && z_repeated {
            z = Some(i);
        }

        if x.is_some() && y.is_some() && z.is_some() { break; }
    }

    (x.unwrap(), y.unwrap(), z.unwrap())
}

fn main() {
    let moons: Vec<Moon> = read_input("input");
    let mut universe1 = Universe::new(moons.clone());
    let universe2 = Universe::new(moons.clone());

    let energy = calc_total_energy(&universe1.nth(999).unwrap());
    println!("Part 1: {}", energy);

    let (x, y, z) = find_universe_cycle(universe2);
    let steps = lcm(x, lcm(y, z));
    println!("Part 2: {}", steps)
}
