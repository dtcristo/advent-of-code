use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::integer::lcm;
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

fn solution(mut input: &str) -> u64 {
    // Build a vector of directions.
    let directions: Vec<Direction> = repeat(0.., Direction::parse)
        .parse_next(&mut input)
        .unwrap();
    // Build the network hash map.
    let network = Network::parse(&mut input).unwrap();

    network
        .map
        // Iterate over map nodes.
        .keys()
        // Find all node nodes ending in "A". These are our "starting nodes".
        .filter(|&node| node[2] == b'A')
        // Find the index where each starting node reaches a node ending in "Z".
        .map(|starting_node| {
            directions
                .iter()
                // Cycle directions infinitely.
                .cycle()
                // Fold to find index of first "Z" node.
                .fold_while((0_u32, starting_node), |(index, node), &direction| {
                    if node[2] == b'Z' {
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
                // Return calculated index as `u64`.
                .0 as u64
        })
        // Calculate the LCM (lowest common multiple) of all indexes.
        .reduce(|acc, index| lcm(acc, index))
        .unwrap()
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
    use crate::*;

    #[test]
    fn test_solution() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
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
