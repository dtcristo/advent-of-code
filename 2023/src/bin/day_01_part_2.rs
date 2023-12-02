fn main() {
    let input = include_bytes!("../../input/day_01");
    let result = solution(input);

    println!("{result}");
}

fn solution(input: &[u8]) -> u32 {
    // Split the input on lines, parse each line into a number and sum.
    input.split(|&byte| byte == b'\n').map(parse_line).sum()
}

fn parse_line(line: &[u8]) -> u32 {
    // Step forward through line one byte at a time testing if any suffix
    // of the current subslice is a valid digit (determined by `parse_digit`).
    // Return the first match.
    let first = (0..=line.len())
        .flat_map(|r| (0..r).map(move |l| &line[l..r]))
        .find_map(parse_digit);

    // Same as above but in the reverse direction.
    let last = (0..=line.len())
        .rev()
        .flat_map(|r| (0..r).rev().map(move |l| &line[l..r]))
        .find_map(parse_digit);

    if first.is_some() && last.is_some() {
        // Join the digis into a number.
        first.unwrap() * 10 + last.unwrap()
    } else {
        0
    }
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
    use rstest::rstest;

    #[test]
    fn test_solution() {
        let input = b"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = solution(input);
        let expected = 281;
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(b"", 0)]
    #[case(b"7pqrstsixteen", 76)]
    #[case(b"ppjvndvknbtpfsncplmhhrlh5", 55)]
    #[case(b"gvzkmxg55twonem", 51)]
    fn test_parse_line(#[case] input: &[u8], #[case] expected: u32) {
        assert_eq!(parse_line(input), expected);
    }
}
