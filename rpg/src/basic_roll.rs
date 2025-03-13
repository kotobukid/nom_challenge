use nom::character::complete::digit1;
use nom::sequence::pair;
use nom::{IResult, Mode, OutputMode, PResult, Parser};

#[derive(Debug, PartialEq)]
pub struct DiceRoll {
    pub sides: u32,
    pub count: u32,
    pub modifier: i32,
}

pub struct BasicRoll {}
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
    let modifier_parser = pair(
        nom::character::complete::one_of("+-"),
        digit1::<&str, nom::error::Error<&str>>,
    );

    let (input, opt_result) = nom::combinator::opt(modifier_parser).parse(input)?;

    match opt_result {
        Some((sign, value)) => {
            let signed_value = match sign {
                '+' => value.parse::<i32>().unwrap_or(0),
                '-' => -1 * value.parse::<i32>().unwrap_or(0),
                _ => unreachable!(),
            };

            Ok((input, signed_value))
        }
        None => Ok((input, 0)),
    }
}
