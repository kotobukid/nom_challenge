use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::{map_res, opt},
    IResult, Parser,
};

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

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("color")(input)?;
    let (input, _) = tag("#")(input)?;

    // 短絡評価があるため、今回は先にrgbaでのパースを試みる必要がある(場合により計算量軽＞重にするなど)
    let (input, color) = alt((hex_rgba, hex_rgb)).parse(input)?;

    Ok((input, color))
}

fn main() {
    println!("{:?}", hex_color("color#2F14DF"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color() {
        assert_eq!(
            hex_color("color#2F14DF"),
            Ok((
                "",
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                    alpha: None,
                }
            ))
        );
    }

    #[test]
    fn parse_color_with_alpha() {
        assert_eq!(
            hex_color("color#2F14DF04"),
            Ok((
                "",
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                    alpha: Some(04_u8),
                }
            ))
        );
    }
}
