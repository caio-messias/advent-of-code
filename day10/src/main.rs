use std::collections::{HashSet, HashMap};
use std::fs;

#[derive(Clone, Copy, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

fn read_input() -> String {
    return fs::read_to_string("input")
        .expect("Failed to read input file.");
}

fn create_asteroids(space: String) -> Vec<Asteroid> {
    space.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, character)| match character {
                    '#' => Some(Asteroid { x: col as i32, y: row as i32 }),
                    _ => None,
                })
        })
        .collect()
}

fn determine_best_location_and_max_seen(asteroids: &[Asteroid]) -> (Asteroid, i32) {
    asteroids.iter()
        .fold((Asteroid { x: 0, y: 0 }, 0), |(best_asteroid, max_seen), &asteroid| {
            let seen_count = calculate_seen_count(asteroids, &asteroid);

            if seen_count > max_seen {
                (asteroid, seen_count)
            } else {
                (best_asteroid, max_seen)
            }
        })
}

fn calculate_seen_count(asteroids: &[Asteroid], central_asteroid: &Asteroid) -> i32 {
    asteroids.iter()
        .filter(|&asteroid| asteroid != central_asteroid)
        .fold(HashSet::new(), |mut seen, asteroid| {
            // Values are subtracted from the central asteroid so we can use it as the origin point.
            let distance_x = asteroid.x - central_asteroid.x;
            let distance_y = asteroid.y - central_asteroid.y;

            let gcd = gcd(distance_x.abs(), distance_y.abs());
            let direction_x = distance_x / gcd;
            let direction_y = distance_y / gcd;

            seen.insert((direction_x, direction_y));
            seen
        }).len() as i32
}

struct VaporizingLaser {
    base_location: Asteroid,
    asteroids_grouped_by_direction: Vec<Vec<Asteroid>>,
    direction: usize,
}

impl VaporizingLaser {
    fn new(asteroids: &[Asteroid], base_location: Asteroid) -> VaporizingLaser {
        let seen = asteroids.iter()
            .filter(|&asteroid| asteroid != &base_location)
            .fold(HashMap::new(), |mut seen, &asteroid| {
                // Values are subtracted from the central asteroid so we can use it as the origin point.
                let distance_x = asteroid.x - base_location.x;
                let distance_y = asteroid.y - base_location.y;
                let gcd = gcd(distance_x.abs(), distance_y.abs());
                let direction_x = distance_x / gcd;
                let direction_y = distance_y / gcd;

                seen.entry((direction_x, direction_y))
                    .or_insert_with(Vec::new)
                    .push(Asteroid { x: distance_x, y: distance_y });

                seen
            });

        let mut asteroids_grouped_by_direction: Vec<Vec<Asteroid>> = seen.values().cloned().collect();

        // Firing the laser in clockwise direction will hit the farthest asteroids first for each direction.
        for asteroids_in_line in asteroids_grouped_by_direction.iter_mut() {
            asteroids_in_line.sort_by_key(|&Asteroid { x, y }| -euclidean_distance(x, y));
        }

        // Sort each group by increasing angle in clockwise direction.
        // atan2 returns angles in counterclockwise direction, so we invert it to get clockwise
        // and call atan2(x, y) rather than atan2(y, x) so the angles starts from the y axis.
        asteroids_grouped_by_direction.sort_by(|direction1, direction2| {
            let Asteroid { x, y } = direction1[0];
            let angle1 = -(x as f64).atan2(y as f64);

            let Asteroid { x, y } = direction2[0];
            let angle2 = -(x as f64).atan2(y as f64);

            angle1.partial_cmp(&angle2).unwrap()
        });

        return VaporizingLaser { base_location, asteroids_grouped_by_direction, direction: 0 };
    }
}

impl Iterator for VaporizingLaser {
    type Item = Asteroid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.asteroids_grouped_by_direction.len() == 0 {
            return None;
        }

        if self.direction == self.asteroids_grouped_by_direction.len() { self.direction = 0 }

        let asteroids = &mut self.asteroids_grouped_by_direction[self.direction];
        let destroyed = asteroids.pop().unwrap();

        let result_x = destroyed.x + self.base_location.x;
        let result_y = destroyed.y + self.base_location.y;

        if asteroids.is_empty() {
            self.asteroids_grouped_by_direction.remove(self.direction);
        } else {
            self.direction += 1;
        }

        return Some(Asteroid { x: result_x, y: result_y });
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    return a;
}

fn euclidean_distance(x: i32, y: i32) -> i64 {
    return (x * x + y * y) as i64;
}

fn main() {
    let asteroid_input = read_input();
    let asteroids = create_asteroids(asteroid_input);

    let (best_asteroid, seen_count) = determine_best_location_and_max_seen(&asteroids);
    println!("Part 1: {}", seen_count);

    let nth_asteroid = VaporizingLaser::new(&asteroids, best_asteroid).nth(199).unwrap();
    println!("Part 2: {}", nth_asteroid.x * 100 + nth_asteroid.y);
}
