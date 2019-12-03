use std::convert::From;
use std::io;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

fn read_input(path: &str) -> io::Result<Vec<String>> {
    return BufReader::new(File::open(path)?)
        .lines().collect();
}

fn calculate_fuel(module: i32) -> i32 {
    return (f64::from(module) / 3f64).floor() as i32 - 2;
}

fn calculate_fuel_with_extra_fuel(module: i32) -> i32 {
    let mut fuel = calculate_fuel(module);
    let mut extra_fuel = calculate_fuel(fuel);

    while extra_fuel > 0 {
        fuel += extra_fuel;
        extra_fuel = calculate_fuel(extra_fuel);
    }

    return fuel;
}

fn calculate_total_fuel(modules: &Vec<i32>, calculate_fuel_method: fn(i32) -> i32) -> i32 {
    return modules.iter()
        .map(|x| calculate_fuel_method(*x))
        .sum()
}
fn main() {
    let modules: Vec<i32> = read_input("input")
        .expect("Could not read the input file")
        .iter()
        .map(|x| i32::from_str(x).unwrap())
        .collect();

    println!("Part 1: {}", calculate_total_fuel(&modules, calculate_fuel));
    println!("Part 2: {}", calculate_total_fuel(&modules, calculate_fuel_with_extra_fuel));
}
