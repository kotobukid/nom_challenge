use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::{map_res, opt};
use nom::{IResult, Mode, OutputMode, PResult, Parser};

pub struct ColorCode {}

impl<'a> Parser<&'a str> for ColorCode {
    type Output = Color;
    type Error = nom::error::Error<&'a str>;

    fn parse(&mut self, input: &'a str) -> IResult<&'a str, Color> {
        hex_color(input)
    }

    fn process<OM: OutputMode>(
        &mut self,
        i: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        match self.parse(i) {
            Ok((remaining, color)) => Ok((remaining, OM::Output::bind(|| color))),
            Err(nom::Err::Error(e)) => Err(nom::Err::Error(OM::Error::bind(|| e))),
            Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(e)),
            Err(nom::Err::Incomplete(n)) => Err(nom::Err::Incomplete(n)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: Option<u8>,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
}

#[allow(dead_code)]
fn hex_primary_opt(input: &str) -> IResult<&str, Option<u8>> {
    opt(hex_primary).parse(input)
}

fn hex_rgb(input: &str) -> IResult<&str, Color> {
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((
        input,
        Color {
            red,
            green,
            blue,
            alpha: None,
        },
    ))
}

fn hex_rgba(input: &str) -> IResult<&str, Color> {
    let (input, (red, green, blue, alpha)) =
        (hex_primary, hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((
        input,
        Color {
            red,
            green,
            blue,
            alpha: Some(alpha),
        },
    ))
}

pub fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = opt(tag("color")).parse(input)?;
    let (input, _) = tag("#")(input)?;

    // 短絡評価があるため、今回は先にrgbaでのパースを試みる必要がある(場合により計算量軽＞重にするなど)
    let (input, color) = alt((hex_rgba, hex_rgb)).parse(input)?;

    Ok((input, color))
}
