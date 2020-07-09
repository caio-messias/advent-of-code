use std::fs;

fn read_input(path: &str) -> (i32, i32) {
    let input: String = fs::read_to_string(path)
        .expect("Failed to read input file.");

    let mut range = input.split("-");
    let lower_bound: i32 = range.next().unwrap().parse().unwrap();
    let upper_bound: i32 = range.next().unwrap().parse().unwrap();

    return (lower_bound, upper_bound);
}

fn digits(i: i32) -> Vec<i32> {
    let mut digits = Vec::with_capacity(6);
    let mut acc = i;

    while acc > 0 {
        digits.push(acc % 10);
        acc /= 10;
    }

    digits.reverse();
    return digits;
}

fn is_password(n: i32, is_strict: bool) -> bool {
    let digits = digits(n);

    return if is_strict {
        is_increasing(&digits) && has_repeating_digits_strict(&digits)
    } else {
        is_increasing(&digits) && has_repeating_digits(&digits)
    }
}

fn is_increasing(digits: &[i32]) -> bool {
    let (mut current_digit, digits) = digits.split_first().unwrap();

    for digit in digits {
        if digit < current_digit {
            return false;
        }
        current_digit = digit;
    }

    return true;
}

fn has_repeating_digits(digits: &[i32]) -> bool {
    let mut repeating_groups = 0;
    let (mut digit_repeating, digits) = digits.split_first().unwrap();

    for digit in digits {
        if digit == digit_repeating {
            repeating_groups += 1;
        }
        digit_repeating = digit;
    }

    return repeating_groups > 0;
}

fn has_repeating_digits_strict(digits: &[i32]) -> bool {
    let mut repeating_digits = 0;
    let (mut last_digit, digits) = digits.split_first().unwrap();
    let mut has_repeating_double_digits = false;

    for current_digit in digits {
        if current_digit == last_digit {
            repeating_digits += 1;
        } else {
            if repeating_digits == 1 {
                has_repeating_double_digits = true
            }
            repeating_digits = 0
        }
        last_digit = current_digit;
    }

    if repeating_digits == 1 {
        has_repeating_double_digits = true
    }

    return has_repeating_double_digits;
}

fn number_of_valid_passwords(lower_bound: i32, upper_bound: i32) -> (i32, i32) {
    let mut valid_passwords = 0;
    let mut valid_passwords_strict = 0;

    for password in lower_bound..upper_bound {
        if is_password(password, false) {
            valid_passwords += 1;
        }
        if is_password(password, true) {
            valid_passwords_strict += 1;
        }
    }

    return (valid_passwords, valid_passwords_strict);
}

fn main() {
    let (lower_bound, upper_bound) = read_input("input");

    let (valid_passwords, valid_passwords_strict) = number_of_valid_passwords(lower_bound, upper_bound);

    println!("Part 1: {}", valid_passwords);
    println!("Part 2: {}", valid_passwords_strict);
}

#[cfg(test)]
pub mod tests {
    use crate::{is_password, digits};

    #[test]
    fn test_digits() {
        let d = digits(123456);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], d);
    }

    #[test]
    fn example1() {
        assert!(is_password(111111, false), "validation failed");
    }

    #[test]
    fn example2() {
        assert!(!is_password(223450, false), "validation failed");
    }

    #[test]
    fn example3() {
        assert!(!is_password(123789, false), "validation failed");
    }

    #[test]
    fn example4() {
        assert!(is_password(112233, true), "validation failed");
    }

    #[test]
    fn example5() {
        assert!(!is_password(123444, true), "validation failed");
    }

    #[test]
    fn example6() {
        assert!(is_password(111122, true), "validation failed");
    }
}
