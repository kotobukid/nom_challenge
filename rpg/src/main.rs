use nom::Parser;

mod basic_roll;
use crate::basic_roll::BasicRoll;

fn main() {
    let mut basic_parser = BasicRoll {};
    let roll = "2D6+";
    let result = basic_parser.parse(roll).unwrap();

    println!(
        "Input: {roll}\nRemaining: {}\nResult: {:?}",
        result.0, result.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_modifier() {
        let mut basic_parser = BasicRoll {};
        let roll = "2D6+4";
        let result = basic_parser.parse(roll);
        assert_eq!(result.unwrap().1.modifier, 4);
    }

    #[test]
    fn test_negative_modifier() {
        let mut basic_parser = BasicRoll {};
        let roll = "5D10-4";
        let result = basic_parser.parse(roll);
        assert_eq!(result.unwrap().1.modifier, -4);
    }

    #[test]
    fn test_no_modifier() {
        let mut basic_parser = BasicRoll {};
        let roll = "5D10";
        assert_eq!(basic_parser.parse(roll).unwrap().1.modifier, 0);
    }

    #[test]
    fn test_modifier_with_sign() {
        let mut basic_parser = BasicRoll {};
        let roll = "5D10+";
        assert!(basic_parser.parse(roll).is_ok());
    }
}
