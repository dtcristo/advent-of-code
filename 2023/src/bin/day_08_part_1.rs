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
    // Build a vector of directions.
    let directions: Vec<Direction> = repeat(0.., Direction::parse)
        .parse_next(&mut input)
        .unwrap();
    // Build the network hash map.
    let network = Network::parse(&mut input).unwrap();

    directions
        .iter()
        // Cycle directions infinitely.
        .cycle()
        // Fold a tuple of `(index: u32, node: &[u8; 3])` representing
        // current index and current node. Keep folding traversing through
        // the map (using current direction) until current node is "ZZZ",
        // incrementing index each step.
        .fold_while((0_u32, b"AAA"), |(index, node), &direction| {
            if node == b"ZZZ" {
                Done((index, node))
            } else {
                // Lookup the connected nodes from the map by node.
                let connected_nodes = network.map.get(node).unwrap();
                Continue((
                    // New index.
                    index + 1,
                    // Get new node from connected nodes by direction.
                    match direction {
                        Direction::Left => &connected_nodes.0,
                        Direction::Right => &connected_nodes.1,
                    },
                ))
            }
        })
        .into_inner()
        // Return calculated index.
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
    // Network map is a hash map from node to 2-tuple of connected nodes
    // (representing left and right direction).
    map: HashMap<[u8; 3], ([u8; 3], [u8; 3])>,
}

impl Network {
    fn new() -> Self {
        Network {
            map: HashMap::new(),
        }
    }

    // Parse an input string slice into `Network`.
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
    use rstest::rstest;

    use crate::*;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        2
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        6
    )]
    fn test_solution(#[case] input: &str, #[case] expected: u32) {
        let result = solution(input);
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
