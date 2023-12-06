use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use winnow::{
    ascii::digit1,
    combinator::alt,
    stream::{Location, Stream},
    token::none_of,
    Located, PResult, Parser,
};

fn main() {
    let mut input = include_str!("../../input/day_03");
    let result = solution(&mut input);
    println!("{result}");
}

fn solution(input: &str) -> u32 {
    let Schematic {
        line_length,
        gear_symbol_indexes,
        numbers,
    } = parse_schematic(input);

    gear_symbol_indexes
        .iter()
        .filter_map(|&index| {
            let unique_numbers = adjacent_indexes(index, line_length)
                .iter()
                .filter_map(|surrounding_index| numbers.get(surrounding_index))
                .collect::<HashSet<&Number>>();

            if unique_numbers.len() == 2 {
                Some(
                    unique_numbers
                        .iter()
                        .map(|number| number.value)
                        .product::<u32>(),
                )
            } else {
                None
            }
        })
        .sum()
}

fn parse_schematic(input: &str) -> Schematic {
    // Iterate lines of input and remember the line length.
    let mut lines_iter = input.lines().peekable();
    let line_length = lines_iter.peek().unwrap().len() + 1;
    // Join the lines into a single string separated by ".".
    let flattened_input = lines_iter.join(".");
    // Wrap in `Located` so parsers can find index within input.
    let mut located_input = Located::new(flattened_input.as_str());

    let mut gear_symbol_indexes = HashSet::new();
    let mut numbers = HashMap::new();

    loop {
        match parse_token.parse_next(&mut located_input) {
            Ok(Token::Number(number)) => {
                (number.index..(number.index + number.length)).for_each(|index| {
                    numbers.insert(index, number.clone());
                })
            }
            Ok(Token::Symbol(symbol)) => {
                if symbol.char == '*' {
                    gear_symbol_indexes.insert(symbol.index);
                }
            }
            Ok(Token::Dot) => {}
            Err(_) => break,
        }
    }

    Schematic {
        line_length,
        gear_symbol_indexes,
        numbers,
    }
}

fn parse_token(input: &mut Located<&str>) -> PResult<Token> {
    alt((parse_number, parse_dot, parse_symbol)).parse_next(input)
}

fn parse_symbol(input: &mut Located<&str>) -> PResult<Token> {
    let index = input.location();
    let char =
        none_of(['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']).parse_next(input)?;

    Ok(Token::Symbol(Symbol { index, char }))
}

fn parse_number(input: &mut Located<&str>) -> PResult<Token> {
    let index = input.location();
    let output = digit1.parse_next(input)?;
    let length = output.len();
    let value = output.parse().unwrap();

    Ok(Token::Number(Number {
        index,
        length,
        value,
    }))
}

fn parse_dot(input: &mut Located<&str>) -> PResult<Token> {
    '.'.parse_next(input)?;

    Ok(Token::Dot)
}

#[derive(Debug, PartialEq, Eq)]
struct Schematic {
    line_length: usize,
    gear_symbol_indexes: HashSet<usize>,
    numbers: HashMap<usize, Number>,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Symbol(Symbol),
    Number(Number),
    Dot,
}

#[derive(Debug, PartialEq, Eq)]
struct Symbol {
    index: usize,
    char: char,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Number {
    index: usize,
    length: usize,
    value: u32,
}

fn adjacent_indexes(index: usize, line_length: usize) -> HashSet<usize> {
    let mut result = HashSet::new();

    let has_space_above = index > line_length;
    let has_space_left = index % line_length != 0;
    let has_space_right = (index + 1) % line_length != 0;

    let left = if has_space_left {
        let left = index - 1;
        result.insert(left);
        left
    } else {
        index
    };

    let right = if has_space_right {
        let right = index + 1;
        result.insert(right);
        right
    } else {
        index
    };

    if has_space_above {
        ((left - line_length)..=(right - line_length)).for_each(|i| {
            result.insert(i);
        });
    }

    ((left + line_length)..=(right + line_length)).for_each(|i| {
        result.insert(i);
    });

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = solution(input);
        let expected = 467835;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_symbol() {
        let input = "*.....";
        let mut located_input = Located::new(input);
        let output = parse_symbol(&mut located_input).unwrap();
        assert_eq!(
            output,
            Token::Symbol(Symbol {
                index: 0,
                char: '*'
            })
        );
        assert_eq!(located_input.finish(), ".....");
    }

    #[test]
    fn test_parse_number() {
        let input = "1234.....";
        let mut located_input = Located::new(input);
        let output = parse_number(&mut located_input).unwrap();
        assert_eq!(
            output,
            Token::Number(Number {
                index: 0,
                length: 4,
                value: 1234,
            })
        );
        assert_eq!(located_input.finish(), ".....");
    }

    #[test]
    fn test_parse_dot() {
        let input = "....1234";
        let mut located_input = Located::new(input);
        let output = parse_dot(&mut located_input).unwrap();
        assert_eq!(output, Token::Dot);
        assert_eq!(located_input.finish(), "...1234");
    }

    #[test]
    fn test_parse_schematic() {
        let input = "467..114..
...*......";
        let output = parse_schematic(input);
        assert_eq!(
            output,
            Schematic {
                line_length: 11,
                gear_symbol_indexes: HashSet::from([14]),
                numbers: HashMap::from([
                    (
                        0,
                        Number {
                            index: 0,
                            length: 3,
                            value: 467,
                        }
                    ),
                    (
                        1,
                        Number {
                            index: 0,
                            length: 3,
                            value: 467,
                        }
                    ),
                    (
                        2,
                        Number {
                            index: 0,
                            length: 3,
                            value: 467,
                        }
                    ),
                    (
                        5,
                        Number {
                            index: 5,
                            length: 3,
                            value: 114,
                        }
                    ),
                    (
                        6,
                        Number {
                            index: 5,
                            length: 3,
                            value: 114,
                        }
                    ),
                    (
                        7,
                        Number {
                            index: 5,
                            length: 3,
                            value: 114,
                        }
                    ),
                ]),
            }
        );
    }
}
