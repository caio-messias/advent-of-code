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

/// Naive decoding: run the algorithm exactly as it was described in the problem.
fn decode_message(signal: Vec<i32>) -> String {
    (0..100)
        .fold(signal, |acc, _| run_phase(acc))
        .into_iter()
        .take(8)
        .map(|d| std::char::from_digit(d as u32, 10).unwrap())
        .collect()
}

/// To decode the full message, due to its size, we need to do some optimizations.
/// 
/// Notice that in the second half of the result message the pattern in entirely composed of 0s and 1s.
/// Since we have an offset that puts us way past the half of the message, we can safely ignore the pattern and only
/// worry with when it will be zero or not. In the full message there won't be a moment where the pattern is -1.
/// 
/// Also, notice that the pattern for the last digit is all 0s except for the last. 
/// As such, the last digit in the new message will be equal to the last digit of the current message.
/// The pattern for the second to last digit is all zeros except for the last two digits.
/// As such, the second to last digit in the new message is the last digit in the new message we just calculated
/// + the second to last digit of the current message.
/// 
/// If we start processing the message backwards, we can store the value of the previous calculation and simply add the current digit
/// to obtain the digit of the new message.
/// 
/// Example with an offset of 4:
/// 0*1  + 3*0  + 4*-1 + 1*0  + 5*1  + 5*0  + 1*-1 + 8*0  = 0
/// 0*0  + 3*1  + 4*1  + 1*0  + 5*0  + 5*-1 + 1*-1 + 8*0  = 1
/// 0*0  + 3*0  + 4*1  + 1*1  + 5*1  + 5*0  + 1*0  + 8*0  = 0
/// 0*0  + 3*0  + 4*0  + 1*1  + 5*1  + 5*1  + 1*1  + 8*0  = 2 << half of the message. Below here the pattern if only 0s and 1s, in this order.
/// 0*0  + 3*0  + 4*0  + 1*0  + 5*1  + 5*1  + 1*1  + 8*1  = 9 << 19 = 5 (current digit) + 14 (just calculated)
/// 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*1  + 1*1  + 8*1  = 4 << 14 = 5 (current digit) + 9 (just calculated) 
/// 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*1  + 8*1  = 9 << 9 = 1 (current digit) + 8 (just calculated) 
/// 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*0  + 8*1  = 8 << only use the last digit of the current message
fn decode_full_message(signal: Vec<i32>) -> String {
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

    let full_message = decode_full_message(full_signal);
    println!("Part 2: {}", full_message);
}

fn vec_to_number(digits: Vec<i32>) -> i64 {
    digits.iter().rev().enumerate().fold(0, |acc, (i, digit)| {
        acc + ((digit * 10i32.pow(i as u32)) as i64)
    })
}


#[cfg(test)]
pub mod tests {
    use crate::{decode_full_message, vec_to_number};

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

        let message = decode_full_message(full_signal);
        assert_eq!(message, "84462026")
    }

    #[test]
    fn part2_example2() {
        let signal = String::from("02935109699940807407585447034323");
        let full_signal = signal.repeat(10000);

        let offset = full_signal.chars().take(7).map(|c| c.to_digit(10).unwrap() as i32).collect();
        let offset = vec_to_number(offset) as usize;

        let full_signal: Vec<i32> = full_signal.chars().skip(offset).map(|c| c.to_digit(10).unwrap() as i32).collect();

        let message = decode_full_message(full_signal);
        assert_eq!(message, "78725270")
    }

    #[test]
    fn part2_example3() {
        let signal = String::from("03081770884921959731165446850517");
        let full_signal = signal.repeat(10000);

        let offset = full_signal.chars().take(7).map(|c| c.to_digit(10).unwrap() as i32).collect();
        let offset = vec_to_number(offset) as usize;

        let full_signal: Vec<i32> = full_signal.chars().skip(offset).map(|c| c.to_digit(10).unwrap() as i32).collect();

        let message = decode_full_message(full_signal);
        assert_eq!(message, "53553731")
    }
}
