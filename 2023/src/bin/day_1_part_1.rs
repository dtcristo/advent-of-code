use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("input/day_1")?;
    let result = solution(input);

    println!("{result}");

    Ok(())
}

fn solution(input: String) -> u32 {
    input
        // Split the input into lines.
        .lines()
        // Map each line into a `u32`.
        .map(|line| {
            // Create an iterator that breaks up the line into characters,
            // filter map each character into a digit (`u32`) returning just
            // the valid digits.
            let mut it = line.chars().filter_map(|char| char.to_digit(10));
            // Get the first digit.
            let first = it.next().unwrap();
            // Get the last digit defaulting to the first if no more digits.
            let last = it.last().unwrap_or(first);

            // Join the digis into the "full" number.
            first * 10 + last
        })
        // Sum the numbers for each line.
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = solution(input.to_string());
        assert_eq!(result, 142);
    }
}
