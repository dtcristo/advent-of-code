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
        .lines()
        .map(|line| {
            let numbers: Vec<char> = line.chars().filter(|char| char.is_numeric()).collect();

            format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
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
