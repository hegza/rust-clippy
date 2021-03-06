// run-rustfix

#![deny(clippy::wildcard_enum_match_arm)]
#![allow(unreachable_code, unused_variables, dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Cyan,
}

impl Color {
    fn is_monochrome(self) -> bool {
        match self {
            Color::Red | Color::Green | Color::Blue => true,
            Color::Rgb(r, g, b) => r | g == 0 || r | b == 0 || g | b == 0,
            Color::Cyan => false,
        }
    }
}

fn main() {
    let color = Color::Rgb(0, 0, 127);
    match color {
        Color::Red => println!("Red"),
        _ => eprintln!("Not red"),
    };
    match color {
        Color::Red => println!("Red"),
        _not_red => eprintln!("Not red"),
    };
    let _str = match color {
        Color::Red => "Red".to_owned(),
        not_red => format!("{:?}", not_red),
    };
    match color {
        Color::Red => {},
        Color::Green => {},
        Color::Blue => {},
        Color::Cyan => {},
        c if c.is_monochrome() => {},
        Color::Rgb(_, _, _) => {},
    };
    let _str = match color {
        Color::Red => "Red",
        c @ Color::Green | c @ Color::Blue | c @ Color::Rgb(_, _, _) | c @ Color::Cyan => "Not red",
    };
    match color {
        Color::Rgb(r, _, _) if r > 0 => "Some red",
        _ => "No red",
    };
    match color {
        Color::Red | Color::Green | Color::Blue | Color::Cyan => {},
        Color::Rgb(..) => {},
    };
    let x: u8 = unimplemented!();
    match x {
        0 => {},
        140 => {},
        _ => {},
    };
}
