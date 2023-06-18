// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let reversed = cc_number.chars().filter(|x| !x.is_whitespace()).rev();
    let mut sum = 0;
    let mut digits_seen = 0;

    for (i, char) in reversed.enumerate() {
        match char.to_digit(10) {
            Some(value) => {
                digits_seen += 1;

                let double = value * 2;
                let char_value = {
                    if (i % 2) == 1 {
                        double / 10 + double % 10
                    } else {
                        value
                    }
                };
                sum += char_value;
            }
            None => return false,
        };
    }

    digits_seen >= 2 && sum % 10 == 0
}

#[cfg(test)]
mod lun_tests {
    use super::*;

    #[test]
    fn test_non_digit_cc_number() {
        assert!(!luhn("foo"));
    }
    #[test]
    fn test_empty_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn(" "));
        assert!(!luhn("  "));
        assert!(!luhn("    "));
    }

    #[test]
    fn test_single_digit_cc_number() {
        assert!(!luhn("0"));
    }

    #[test]
    fn test_two_digit_cc_number() {
        assert!(luhn(" 0 0 "));
    }

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
}

#[allow(dead_code)]
fn main() {}
