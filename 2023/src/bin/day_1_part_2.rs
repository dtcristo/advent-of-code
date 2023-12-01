use anyhow::Result;
use std::fs::read;

fn main() -> Result<()> {
    let input = read("input/day_1")?;
    let result = solution(input);

    println!("{result}");

    Ok(())
}

fn solution(input: Vec<u8>) -> u32 {
    input
        .split(|&byte| byte == b'\n')
        .map(parse_line)
        .flatten()
        .map(|(first, last)| first * 10 + last)
        .sum()
}

fn parse_line(line: &[u8]) -> Option<(u32, u32)> {
    let first = parse_line_first(line);
    let last = parse_line_last(line);

    if first.is_some() && last.is_some() {
        Some((first.unwrap(), last.unwrap()))
    } else {
        None
    }
}

fn parse_line_first(line: &[u8]) -> Option<u32> {
    for right in 0..=line.len() {
        for left in 0..right {
            if let Some(x) = parse_digit(&line[left..right]) {
                return Some(x);
            }
        }
    }

    None
}

fn parse_line_last(line: &[u8]) -> Option<u32> {
    for right in (0..=line.len()).rev() {
        for left in (0..right).rev() {
            if let Some(x) = parse_digit(&line[left..right]) {
                return Some(x);
            }
        }
    }

    None
}

fn parse_digit(input: &[u8]) -> Option<u32> {
    match input {
        b"1" | b"one" => Some(1),
        b"2" | b"two" => Some(2),
        b"3" | b"three" => Some(3),
        b"4" | b"four" => Some(4),
        b"5" | b"five" => Some(5),
        b"6" | b"six" => Some(6),
        b"7" | b"seven" => Some(7),
        b"8" | b"eight" => Some(8),
        b"9" | b"nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = b"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = solution(input.to_vec());
        let expected = 281;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line() {
        let input = b"7pqrstsixteen";
        let result = parse_line(input);
        let expected = Some((7, 6));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line_2() {
        let input = b"ppjvndvknbtpfsncplmhhrlh5";
        let result = parse_line(input);
        let expected = Some((5, 5));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line_3() {
        let input = b"";
        let result = parse_line(input);
        let expected = None;
        assert_eq!(result, expected);
    }
}
