mod color_code;

use crate::color_code::{ColorCode};
use nom::{Parser};

fn main() {
    // ColorCode構造体のインスタンスを作成
    let mut parser = ColorCode {};

    // 入力文字列
    let input = "color#2F14DF11";

    // パースの実行
    match parser.parse(input) {
        Ok((remaining, color)) => {
            println!("Parsed color: {:?}", color);
            println!("Remaining input: {:?}", remaining);
        }
        Err(err) => {
            eprintln!("Failed to parse input: {:?}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color_code::{hex_color, Color};
    #[allow(unused_imports)]
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
    fn parse_color_e1() {
        assert!(hex_color("color#2F14DG").is_err(),);
    }

    #[test]
    fn parse_color_e2() {
        assert!(hex_color("olor#2F14DF").is_err(),);
    }

    #[test]
    fn parse_color_without_func_name() {
        assert_eq!(
            hex_color("#2F14DF"),
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
