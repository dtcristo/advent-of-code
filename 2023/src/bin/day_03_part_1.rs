use std::{collections::HashSet, hash::Hash, num::Saturating};

use winnow::{
    ascii::digit1,
    combinator::alt,
    stream::{Location, Stream},
    token::any,
    Located, PResult, Parser,
};

fn main() {
    let mut input = include_str!("../../input/day_03");
    let result = solution(&mut input);

    println!("{result}");
}

fn solution(input: &str) -> u32 {
    let schematic = parse_schematic(input);
    4361
}

fn parse_schematic(input: &str) -> Schematic {
    let mut lines_iter = input.lines().peekable();
    let line_length = lines_iter.peek().unwrap().len();
    let flattened_input: String = lines_iter.collect();
    let mut located_input = Located::new(flattened_input.as_str());

    let mut symbol_indexes = HashSet::new();
    let mut numbers = vec![];

    loop {
        match parse_token.parse_next(&mut located_input) {
            Ok(Token::Number(number)) => numbers.push(number),
            Ok(Token::Symbol(symbol)) => {
                symbol_indexes.insert(symbol.index);
            }
            Ok(Token::Dot) => {}
            Err(_) => break,
        }
    }

    Schematic {
        line_length,
        symbol_indexes,
        numbers,
    }
}

fn parse_token(input: &mut Located<&str>) -> PResult<Token> {
    alt((parse_number, parse_dot, parse_symbol)).parse_next(input)
}

fn parse_symbol(input: &mut Located<&str>) -> PResult<Token> {
    let index = input.location();
    any.parse_next(input)?;

    dbg!(input);

    Ok(Token::Symbol(Symbol { index }))
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
    symbol_indexes: HashSet<usize>,
    numbers: Vec<Number>,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Symbol(Symbol),
    Number(Number),
    Dot,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol {
    index: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Number {
    index: usize,
    length: usize,
    value: u32,
}

impl Number {
    fn surrounding_indexes(&self, line_length: usize) -> HashSet<usize> {
        let mut result = HashSet::new();
        result.insert(line_length);
        result
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::*;
    use rstest::rstest;

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
        let expected = 4361;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_symbol() {
        let input = "*.....";
        let mut located_input = Located::new(input);
        let output = parse_symbol(&mut located_input).unwrap();
        assert_eq!(output, Token::Symbol(Symbol { index: 0 }));
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
        dbg!(located_input.location());
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
...*......
..35.#633.";
        let output = parse_schematic(input);
        assert_eq!(
            output,
            Schematic {
                line_length: 10,
                symbol_indexes: HashSet::from([13, 25]),
                numbers: vec![
                    Number {
                        index: 0,
                        length: 3,
                        value: 467,
                    },
                    Number {
                        index: 5,
                        length: 3,
                        value: 114,
                    },
                    Number {
                        index: 22,
                        length: 2,
                        value: 35,
                    },
                    Number {
                        index: 26,
                        length: 3,
                        value: 633,
                    }
                ],
            }
        );
    }

    #[rstest]
    #[case(
        Number {
            index: 5,
            length: 3,
            value: 114,
        },
        10,
        HashSet::from([4, 8, 14, 15, 16, 17, 18]),
    )]
    #[case(
        Number {
            index: 98,
            length: 2,
            value: 88,
        },
        10,
        HashSet::from([87, 88, 89, 97, 107, 108, 109]),
    )]
    fn test_number_surrounding_indexes(
        #[case] number: Number,
        #[case] line_length: usize,
        #[case] expected: HashSet<usize>,
    ) {
        let result = number.surrounding_indexes(line_length);
        assert_eq!(result, expected);
    }
}
