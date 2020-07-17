use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::fs;

use regex::Regex;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Material {
    name: String,
    quantity: u64,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Synthesis {
    output: Material,
    requirements: VecDeque<Material>,
}

fn read_ingredients(path: &str) -> HashMap<String, Synthesis> {
    let input: String = fs::read_to_string(path)
        .expect("Failed to read input file.");

    // split left => right
    let line_regex = Regex::new(r"^(.+) => (.+)$").unwrap();
    // extract <quantity> <name>
    let material_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    return input
        .lines()
        .fold(HashMap::new(), |mut reactions: HashMap<String, Synthesis>, line| {
            let line_capture = line_regex.captures(&line).unwrap();
            let input_chemicals = &line_capture[1];
            let output_chemical = &line_capture[2];

            let mut output_requirement: VecDeque<Material> = VecDeque::new();
            for material in material_regex.captures_iter(input_chemicals) {
                let quantity: u64 = material[1].parse().unwrap();
                let name = material[2].to_string();
                output_requirement.push_back(Material { name, quantity });
            }

            let output_capture = material_regex.captures(output_chemical).unwrap();
            let quantity: u64 = output_capture[1].parse().unwrap();
            let name = output_capture[2].to_string();
            let output_material = Material { name: name.clone(), quantity };
            let output_reaction = Synthesis { output: output_material, requirements: output_requirement };
            reactions.insert(name, output_reaction);
            reactions
        });
}

fn try_leftover(inventory: &mut HashMap<String, u64>, cost: &Material) -> u64 {
    let mut needed_quantity = cost.quantity;

    inventory.entry(String::from(&cost.name))
        .and_modify(|available| {
            if *available >= cost.quantity {
                needed_quantity = 0;
                *available -= cost.quantity;
            } else {
                needed_quantity -= *available;
                *available = 0;
            }
        });

    return needed_quantity;
}

fn calc_fuel_cost(reactions: &HashMap<String, Synthesis>, fuel_amount: u64) -> u64 {
    let mut total_cost: VecDeque<Material> = VecDeque::new();
    total_cost.push_back(Material { name: String::from("FUEL"), quantity: fuel_amount });

    let mut inventory: HashMap<String, u64> = HashMap::new();
    let mut ore_cost = 0;


    while let Some(cost) = total_cost.pop_front() {
        if &cost.name == "ORE" {
            ore_cost += cost.quantity;
        } else {
            let cost_synthesis = reactions.get(&cost.name).unwrap();

            let needed_quantity = try_leftover(&mut inventory, &cost);
            if needed_quantity == 0 { continue; }

            let number_of_reactions = f64::ceil((needed_quantity as f64) / (cost_synthesis.output.quantity as f64)) as u64;

            let leftover = number_of_reactions * cost_synthesis.output.quantity - needed_quantity;
            inventory.entry(String::from(&cost.name))
                .and_modify(|available| *available += leftover)
                .or_insert(leftover);

            for material in &cost_synthesis.requirements {
                total_cost.push_back(Material {
                    name: String::from(&material.name),
                    quantity: number_of_reactions * material.quantity,
                })
            }
        }
    }

    return ore_cost;
}

fn max_fuel(reactions: &HashMap<String, Synthesis>, ore_amount: u64) -> u64 {
    let one_fuel_cost = calc_fuel_cost(&reactions, 1);
    let mut fuel_left = ore_amount / one_fuel_cost;
    let mut fuel_right = ore_amount;

    while fuel_right - fuel_left > 1 {
        let fuel_mid = (fuel_left + fuel_right) / 2;
        let fuel_mid_cost = calc_fuel_cost(&reactions, fuel_mid);

        if fuel_mid_cost < ore_amount {
            fuel_left = fuel_mid;
        } else {
            fuel_right = fuel_mid;
        }
    }

    return fuel_left;
}

fn main() {
    let reactions = read_ingredients("input");

    println!("Part 1: {}", calc_fuel_cost(&reactions, 1));
    println!("part 2: {}", max_fuel(&reactions, 1000000000000));
}

#[cfg(test)]
pub mod tests {
    use crate::{calc_fuel_cost, max_fuel, read_ingredients};

    #[test]
    fn test_example1() {
        let reactions = read_ingredients("input_test1");
        assert_eq!(calc_fuel_cost(&reactions, 1), 31);
    }

    #[test]
    fn test_example2() {
        let reactions = read_ingredients("input_test2");
        assert_eq!(calc_fuel_cost(&reactions, 1), 165);
    }

    #[test]
    fn test_example3() {
        let reactions = read_ingredients("input_test3");
        assert_eq!(calc_fuel_cost(&reactions, 1), 13312);
        assert_eq!(max_fuel(&reactions, 1000000000000), 82892753);
    }

    #[test]
    fn test_example4() {
        let reactions = read_ingredients("input_test4");
        assert_eq!(calc_fuel_cost(&reactions, 1), 180697);
        assert_eq!(max_fuel(&reactions, 1000000000000), 5586022);
    }

    #[test]
    fn test_example5() {
        let reactions = read_ingredients("input_test5");
        assert_eq!(calc_fuel_cost(&reactions, 1), 2210736);
        assert_eq!(max_fuel(&reactions, 1000000000000), 460664);
    }
}
