use nom::combinator::opt;
use nom::{IResult, Mode, OutputMode, PResult, Parser};

#[derive(Debug, PartialEq)]
struct DiceRoll {
    sides: u8,
    count: u8,
    modifier: i8,
}

struct BasicRoll {}
impl<'a> Parser<&'a str> for BasicRoll {
    type Output = DiceRoll;
    type Error = nom::error::Error<&'a str>;

    fn parse(&mut self, input: &'a str) -> IResult<&'a str, DiceRoll> {
        let (input, count) = nom::character::complete::digit1(input)?;
        let (input, _) = nom::character::complete::char('D')(input)?;
        let (input, sides) = nom::character::complete::digit1(input)?;
        let (input, sign) = opt(nom::character::complete::one_of("-+")).parse(input)?;
        let (input, modifier) = opt(nom::character::complete::digit1).parse(input)?;

        let mut modifier_default = String::new();
        if let Some(sign) = sign {
            if let Some(modifier) = modifier {
                modifier_default = sign.to_string() + modifier;
            }
        }

        Ok((
            input,
            DiceRoll {
                sides: sides.parse().unwrap(),
                count: count.parse().unwrap(),
                modifier: modifier_default.parse().unwrap(),
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

fn main() {
    let mut basic_parser = BasicRoll {};
    let roll = "2D6+4";
    let result = basic_parser.parse(roll);
    println!("{:?}", result);
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
}
