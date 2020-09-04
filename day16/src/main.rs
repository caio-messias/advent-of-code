use std::fs;
use std::iter;

fn run_phase(signal: Vec<i32>) -> Vec<i32> {
    signal.iter()
        .zip(1..)
        .map(|(_, i)| {
            signal.iter()
                .zip(pattern(i))
                .fold(0, |acc, (s, p)| acc + s*p)
        })
        .map(|x| x.abs())
        .map(|x| x % 10)
        .collect()
}

fn pattern(i: usize) -> impl Iterator<Item = i32> {
    return iter::repeat(0).take(i)
        .chain(iter::repeat(1).take(i))
        .chain(iter::repeat(0).take(i))
        .chain(iter::repeat(-1).take(i))
        .cycle()
        .skip(1);
}

fn decode_message(signal: Vec<i32>) -> String {
    (0..100)
        .fold(signal, |acc, _| run_phase(acc))
        .into_iter()
        .take(8)
        .map(|d| std::char::from_digit(d as u32, 10).unwrap())
        .collect()
}

fn decode_message_full(signal: Vec<i32>) -> String {
    let len = signal.len();

    (0..100).fold(signal, |acc, _| {
        acc.iter().rev()
            .fold((Vec::with_capacity(len), 0), |(mut acc_arr, acc), value| {
                acc_arr.push(acc + value);
                (acc_arr, acc + value)
            })
            .0.iter().rev()
            .map(|x| x.abs())
            .map(|x| x % 10)
            .collect()
    })
    .into_iter()
    .take(8)
    .map(|d| std::char::from_digit(d as u32, 10).unwrap())
    .collect()
}

fn main() {
    let signal = fs::read_to_string("input").unwrap();
    let full_signal = signal.repeat(10000);

    let offset = full_signal.chars().take(7).map(|c| c.to_digit(10).unwrap() as i32).collect();
    let offset = vec_to_number(offset) as usize;

    let signal: Vec<i32> = signal.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let full_signal: Vec<i32> = full_signal.chars().skip(offset).map(|c| c.to_digit(10).unwrap() as i32).collect();

    let message = decode_message(signal);
    println!("Part 1: {}", message);

    let full_message = decode_message_full(full_signal);
    println!("Part 2: {}", full_message);
    // 17069048
}

fn vec_to_number(digits: Vec<i32>) -> i64 {
    digits.iter().rev().enumerate().fold(0, |acc, (i, digit)| {
        acc + ((digit * 10i32.pow(i as u32)) as i64)
    })
}


#[cfg(test)]
pub mod tests {
    use crate::{decode_message_full, vec_to_number};

    #[test]
    fn vec_9834_should_return_number_9834() {
        let digits = vec![9, 8, 3, 4];
        assert_eq!(vec_to_number(digits), 9834);
    }

    #[test]
    fn part2_example1() {
        let signal = String::from("03036732577212944063491565474664");
        let full_signal = signal.repeat(10000);

        let offset = full_signal.chars().take(7).map(|c| c.to_digit(10).unwrap() as i32).collect();
        let offset = vec_to_number(offset) as usize;

        let full_signal: Vec<i32> = full_signal.chars().skip(offset).map(|c| c.to_digit(10).unwrap() as i32).collect();

        let message = decode_message_full(full_signal);
        assert_eq!(message, "84462026")
    }

    #[test]
    fn part2_example2() {
        let signal = String::from("02935109699940807407585447034323");
        let full_signal = signal.repeat(10000);

        let offset = full_signal.chars().take(7).map(|c| c.to_digit(10).unwrap() as i32).collect();
        let offset = vec_to_number(offset) as usize;

        let full_signal: Vec<i32> = full_signal.chars().skip(offset).map(|c| c.to_digit(10).unwrap() as i32).collect();

        let message = decode_message_full(full_signal);
        assert_eq!(message, "78725270")
    }

    #[test]
    fn part2_example3() {
        let signal = String::from("03081770884921959731165446850517");
        let full_signal = signal.repeat(10000);

        let offset = full_signal.chars().take(7).map(|c| c.to_digit(10).unwrap() as i32).collect();
        let offset = vec_to_number(offset) as usize;

        let full_signal: Vec<i32> = full_signal.chars().skip(offset).map(|c| c.to_digit(10).unwrap() as i32).collect();

        let message = decode_message_full(full_signal);
        assert_eq!(message, "53553731")
    }
}
