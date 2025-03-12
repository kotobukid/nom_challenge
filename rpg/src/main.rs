use nom::character::complete::digit1;
use nom::sequence::pair;
use nom::{IResult, Mode, OutputMode, PResult, Parser};

#[derive(Debug, PartialEq)]
struct DiceRoll {
    sides: u32,
    count: u32,
    modifier: i32,
}

struct BasicRoll {}
impl<'a> Parser<&'a str> for BasicRoll {
    type Output = DiceRoll;
    type Error = nom::error::Error<&'a str>;

    fn parse(&mut self, input: &'a str) -> IResult<&'a str, DiceRoll> {
        let (input, count) = digit1(input)?;
        let (input, _) = nom::character::complete::char('D')(input)?;
        let (input, sides) = digit1(input)?;
        let (input, modifier) = parse_modifier(input)?;

        Ok((
            input,
            DiceRoll {
                sides: sides.parse().unwrap(),
                count: count.parse().unwrap(),
                modifier,
            },
        ))
    }

    fn process<OM: OutputMode>(
        &mut self,
        i: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        match self.parse(i) {
            Ok((remaining, d_roll)) => Ok((remaining, OM::Output::bind(|| d_roll))),
            Err(nom::Err::Error(e)) => Err(nom::Err::Error(OM::Error::bind(|| e))),
            Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(e)),
            Err(nom::Err::Incomplete(n)) => Err(nom::Err::Incomplete(n)),
        }
    }
}

fn parse_modifier(input: &str) -> IResult<&str, i32> {
    let res = pair(
        nom::character::complete::one_of("+-"),
        digit1::<&str, nom::error::Error<&str>>,
    )
    .parse(input);

    match res {
        Ok((input, (sign, value))) => {
            let signed_value = match sign {
                '+' => value.parse::<i32>().unwrap_or(0),
                '-' => -1 * value.parse::<i32>().unwrap_or(0),
                _ => unreachable!(),
            };

            Ok((input, signed_value))
        }
        Err(_e) => {
            // eprintln!("{:?}", e);
            // Err(e)
            Ok((input, 0))
        }
    }
}

fn main() {
    let mut basic_parser = BasicRoll {};
    let roll = "2D6+1";
    let result = basic_parser.parse(roll);
    println!("{:?}", result.unwrap().1);
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
        assert!(basic_parser.parse(roll).is_err());
    }
}
