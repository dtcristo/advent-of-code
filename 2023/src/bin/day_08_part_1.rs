use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use winnow::{
    combinator::{fail, fold_repeat, opt, repeat},
    token::{any, take},
    PResult, Parser,
};

fn main() {
    let input = include_str!("../../input/day_08");
    let result = solution(input);
    println!("{result}");
}

fn solution(mut input: &str) -> u32 {
    let directions: Vec<_> = repeat(0.., Direction::parse)
        .parse_next(&mut input)
        .unwrap();
    let network = Network::parse(&mut input).unwrap();

    directions
        .iter()
        .cycle()
        .fold_while((0_u32, b"AAA"), |(index, location), &direction| {
            if location == b"ZZZ" {
                Done((index, location))
            } else {
                Continue((
                    index + 1,
                    match direction {
                        Direction::Left => &network.map.get(location).unwrap().0,
                        Direction::Right => &network.map.get(location).unwrap().1,
                    },
                ))
            }
        })
        .into_inner()
        .0
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    // Parse an input string slice into `Direction`.
    fn parse(input: &mut &str) -> PResult<Direction> {
        Ok(match any.parse_next(input)? {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return fail(input),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Network {
    map: HashMap<[u8; 3], ([u8; 3], [u8; 3])>,
}

impl Network {
    fn new() -> Self {
        Network {
            map: HashMap::new(),
        }
    }

    // Parse an input string slice into `Direction`.
    fn parse(input: &mut &str) -> PResult<Network> {
        let _ = "\n\n".parse_next(input)?;
        fold_repeat(
            0..,
            (
                take(3_usize),
                take(4_usize),
                take(3_usize),
                take(2_usize),
                take(3_usize),
                take(1_usize),
                opt('\n'),
            ),
            Network::new,
            |mut network: Network,
             (key, _, left, _, right, _, _): (&str, _, &str, _, &str, _, _)| {
                network.map.insert(
                    key.as_bytes().try_into().unwrap(),
                    (
                        left.as_bytes().try_into().unwrap(),
                        right.as_bytes().try_into().unwrap(),
                    ),
                );
                network
            },
        )
        .parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = solution(input);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solution_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = solution(input);
        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_direction_parse() {
        let mut input = "RLLR";
        let result = Direction::parse(&mut input).unwrap();
        assert_eq!(result, Direction::Right);
        assert_eq!(input, "LLR");
    }

    #[test]
    fn test_network_parse() {
        let mut input = "

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)";
        let result = Network::parse(&mut input).unwrap();
        assert_eq!(
            result,
            Network {
                map: HashMap::from([
                    (b"AAA".to_owned(), (b"BBB".to_owned(), b"CCC".to_owned())),
                    (b"BBB".to_owned(), (b"DDD".to_owned(), b"EEE".to_owned())),
                    (b"CCC".to_owned(), (b"ZZZ".to_owned(), b"GGG".to_owned())),
                ])
            }
        );
        assert_eq!(input, "");
    }
}
