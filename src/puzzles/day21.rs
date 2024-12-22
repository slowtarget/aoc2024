//--- Day 21: Keypad Conundrum ---

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use lazy_static::lazy_static;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use DirectionPad::{Up, Down, Left, Right, Push};
use KeyPad::{Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, KeyA};
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Copy, Clone, Debug)]
enum DirectionPad {
    Up,
    Down,
    Left,
    Right,
    Push
}
impl DirectionPad {
    fn new() -> Self {
        Push
    }
    fn push(&mut self, target: DirectionPad) -> Box<[DirectionPad]> {
        let moves = match self {
            Up => match target {
                Up => vec![],
                Down => vec![Down],
                Left => vec![Down, Left],
                Right => vec![Down, Right],
                Push => vec![Right],
            },
            Down => match target {
                Up => vec![Up],
                Down => vec![],
                Left => vec![Left],
                Right => vec![Right],
                Push => vec![Up, Right],
            },
            Left => match target {
                Up => vec![Right, Up],
                Down => vec![Right],
                Left => vec![],
                Right => vec![Right, Right],
                Push => vec![Right, Right, Up],
            },
            Right => match target {
                Up => vec![Left, Up],
                Down => vec![Left],
                Left => vec![Left, Left],
                Right => vec![],
                Push => vec![Up],
            },
            Push => match target {
                Up => vec![Left],
                Down => vec![Left, Down],
                Left => vec![Down, Left, Left],
                Right => vec![Down],
                Push => vec![],
            },
        };
        *self = target;
        Vec::from_iter(moves.into_iter().chain(std::iter::once(Push))).into_boxed_slice()
    }
}
impl fmt::Display for DirectionPad {
    // This method controls how the enum is formatted as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Up => write!(f, "^"),
            Down => write!(f, "v"),
            Left => write!(f, "<"),
            Right => write!(f, ">"),
            Push => write!(f, "A"),
        }
    }
}
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
#[derive(Copy, Clone, Debug)]
enum KeyPad {
    KeyA,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
}
impl KeyPad {
    fn new() -> Self {
        KeyPad::KeyA
    }
    
    fn push(&mut self, target: KeyPad) -> Box<[DirectionPad]> {
        let moves: Vec<DirectionPad> = match self {
            Key1 => match target {
                Key1 => vec![],
                Key2 => vec![Right],
                Key3 => vec![Right, Right],
                Key4 => vec![Up],
                Key5 => vec![Up, Right],
                Key6 => vec![Up, Right, Right],
                Key7 => vec![Up, Up],
                Key8 => vec![Up, Up, Right],
                Key9 => vec![Up, Up, Right, Right],
                Key0 => vec![Right, Down],
                KeyA => vec![Right, Right, Down],
            },
            Key2 => match target {
                Key1 => vec![Left],
                Key2 => vec![],
                Key3 => vec![Right],
                Key4 => vec![Left, Up],
                Key5 => vec![Up],
                Key6 => vec![Up, Right],
                Key7 => vec![Left, Up, Up],
                Key8 => vec![Up, Up],
                Key9 => vec![Up, Up, Right],
                Key0 => vec![Down],
                KeyA => vec![Right, Down],
            },
            Key3 => match target {
                Key1 => vec![Left, Left],
                Key2 => vec![Left],
                Key3 => vec![],
                Key4 => vec![Left, Left, Up],
                Key5 => vec![Left, Up],
                Key6 => vec![Up],
                Key7 => vec![Left, Left,Up, Up],
                Key8 => vec![Left, Up, Up, ],
                Key9 => vec![Up, Up],
                Key0 => vec![Down, Left],
                KeyA => vec![Down],
            },
            Key4 => match target {
                Key1 => vec![Down],
                Key2 => vec![Right, Down],
                Key3 => vec![Right, Right, Down],
                Key4 => vec![],
                Key5 => vec![Right],
                Key6 => vec![Right, Right],
                Key7 => vec![Up],
                Key8 => vec![Up, Right],
                Key9 => vec![Up, Right, Right],
                Key0 => vec![Right, Down,  Down],
                KeyA => vec![Right, Right, Down, Down],
            },
            Key5 => match target {
                Key1 => vec![Down, Left],
                Key2 => vec![Down],
                Key3 => vec![Right, Down],
                Key4 => vec![Left],
                Key5 => vec![],
                Key6 => vec![Right],
                Key7 => vec![Left, Up],
                Key8 => vec![Up],
                Key9 => vec![Up, Right],
                Key0 => vec![Down, Down],
                KeyA => vec![Right, Down, Down],
            },
            Key6 => match target {
                Key1 => vec![Down, Left, Left],
                Key2 => vec![Down, Left],
                Key3 => vec![Down],
                Key4 => vec![Left, Left],
                Key5 => vec![Left],
                Key6 => vec![],
                Key7 => vec![ Left, Left, Up],
                Key8 => vec![Left, Up],
                Key9 => vec![Up],
                Key0 => vec![Down, Down, Left],
                KeyA => vec![Down, Down],
            },
            Key7 => match target {
                Key1 => vec![Down, Down],
                Key2 => vec![Right, Down, Down],
                Key3 => vec![Right, Right, Down, Down, ],
                Key4 => vec![Down],
                Key5 => vec![Right, Down, ],
                Key6 => vec![ Right, Right, Down,],
                Key7 => vec![],
                Key8 => vec![Right],
                Key9 => vec![Right, Right],
                Key0 => vec![Right, Down,  Down, Down],
                KeyA => vec![Right, Right, Down, Down, Down],
            },
            Key8 => match target {
                Key1 => vec![Down, Down, Left],
                Key2 => vec![Down, Down],
                Key3 => vec![Right, Down, Down],
                Key4 => vec![Down, Left],
                Key5 => vec![Down],
                Key6 => vec![Right, Down],
                Key7 => vec![Left],
                Key8 => vec![],
                Key9 => vec![Right],
                Key0 => vec![Down, Down, Down],
                KeyA => vec![Right, Down, Down, Down],
            },
            Key9 => match target {
                Key1 => vec![Down, Down, Left, Left],
                Key2 => vec![Down, Down, Left],
                Key3 => vec![Down, Down],
                Key4 => vec![Down, Left, Left],
                Key5 => vec![Down, Left],
                Key6 => vec![Down],
                Key7 => vec![Left, Left],
                Key8 => vec![Left],
                Key9 => vec![],
                Key0 => vec![Down, Down, Down, Left],
                KeyA => vec![Down, Down, Down],
            },
            Key0 => match target {
                Key1 => vec![Up, Left],
                Key2 => vec![Up],
                Key3 => vec![Up, Right],
                Key4 => vec![Up, Up, Left],
                Key5 => vec![Up, Up],
                Key6 => vec![Up, Up, Right],
                Key7 => vec![Up, Up, Up, Left],
                Key8 => vec![Up, Up, Up],
                Key9 => vec![Up, Up, Up, Right],
                Key0 => vec![],
                KeyA => vec![Right],
            },
            KeyA => match target {
                Key1 => vec![Up, Left, Left],
                Key2 => vec![Left, Up],
                Key3 => vec![Up],
                Key4 => vec![Up, Up, Left, Left],
                Key5 => vec![Left, Up, Up],
                Key6 => vec![Up, Up],
                Key7 => vec![Up, Up, Up, Left, Left],
                Key8 => vec![Left, Up, Up, Up],
                Key9 => vec![Up, Up, Up],
                Key0 => vec![Left],
                KeyA => vec![],
            },
        };
        *self = target;
        Vec::from_iter(moves.into_iter().chain(std::iter::once(Push))).into_boxed_slice()
    }
}

lazy_static! {
    static ref KEYPAD_MAP: HashMap<usize, KeyPad> = {
        let data: [(usize, KeyPad); 10] = [
            (1, Key1),
            (2, Key2),
            (3, Key3),
            (4, Key4),
            (5, Key5),
            (6, Key6),
            (7, Key7),
            (8, Key8),
            (9, Key9),
            (0, Key0),
        ];
        data.into_iter().collect()
    };
}

// Helper function to parse a single unsigned integer
fn parse_unsigned(input: &str) -> IResult<&str, usize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        usize::from_str(s)
    })(input)?;

    Ok((i, number))
}
fn parse_code(input: &str) -> IResult<&str, usize>  {
    let (i, result) = parse_unsigned(input)?;
    let (i, _) = tag("A")(i)?;
    Ok((i, result))
}
fn parse(input: String) -> Vec<usize> {
    separated_list1(tag("\n"), parse_code)(input.trim()).unwrap().1
}
fn three_digits_string(n: usize) -> [usize; 3] {
    let s = format!("{:03}", n); // always 3 digits with leading zeros if needed
    let mut digits = [0; 3];
    for (i, c) in s.chars().enumerate() {
        // Each char is guaranteed to be '0'..='9', so unwrap is safe
        digits[i] = c.to_digit(10).unwrap() as usize;
    }
    digits
}
fn get_complexity(code: usize) -> i32 {
    let mut door = KeyPad::new();
    let mut robot1 = DirectionPad::new();
    let mut robot2 = DirectionPad::new();
    let mut keys = three_digits_string(code).iter().map(|&d| *KEYPAD_MAP.get(&d).unwrap()).collect::<Vec<_>>();
    keys.push(KeyA);
    println!("{:?}", keys);
    let me = keys.iter().flat_map(|k| door.push(*k))
        .flat_map(|k| robot1.push(k))
        .flat_map(|k| robot2.push(k))
        .collect::<Vec<_>>();

    println!("{}", me.iter().map(|d| d.to_string()).collect::<String>());
    println!("{}: {:?}", code, me.len());
    me.len() as i32 * code as i32
}
pub(crate) fn solve(input: String) -> (i32, i32) {
    let codes = parse(input);
    println!("{:?}", codes);

    let mut sum = 0;
    for code in codes {
        sum += get_complexity(code);
    }

    (sum,1) // 439_726 is too high // 180_204 is too high // 176_964 is too high
}
#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
    static ref TEST_MAP: HashMap<usize, &'static str> = {
        // We can build from an array of tuples:
        let data: [(usize, &str); 5] = [
    (029,"<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"),
    (980,"<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A"),
    (179,"<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
    (456,"<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"),
    (379,"<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
        ];
        data.into_iter().collect()
    };
}

    #[test]
    fn test_parse() {
        let input = "029A".to_string();
        let expected = vec![29];
        assert_eq!(parse(input), expected);
    }
    #[test]
    fn test_three_digits_string() {
        assert_eq!(three_digits_string(1), [0, 0, 1]);
        assert_eq!(three_digits_string(12), [0, 1, 2]);
        assert_eq!(three_digits_string(123), [1, 2, 3]);
    }

    #[test]
    fn test_test_map() {
        for (code, expected) in TEST_MAP.iter() {
            println!(" testing: {:?} expected: {}", code, expected.len());
            println!("{}", expected);
            assert_eq!(get_complexity(*code), expected.len() as i32 * *code as i32);
        }
    }
    // [Key3, Key7, Key9, KeyA] expected 64 was 68
    // <v<A>>^AvA^A<v A<  AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v< A>A>^AAAvA<^A>A
    // v<<A>>^AvA^Av<<A>>^AA<vA<A>>^AAvAA<^A>A<vA^>AA<A>Av<<A>A^>AAA<Av>A^A
}


