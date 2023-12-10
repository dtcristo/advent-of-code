use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/day_04");
    let result = solution(input);
    println!("{result}");
}

fn solution(input: &str) -> u32 {
    let mut card_counts = vec![1_u32; input.lines().count()];

    input
        .lines()
        .map(|line| {
            let mut iter = line.split(':');
            iter.next();
            iter.next()
                .unwrap()
                .split('|')
                .map(|numbers| {
                    numbers
                        .split_ascii_whitespace()
                        .map(|number| number.parse().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .tuple_windows()
                .map(|(winning, mine)| winning.intersection(&mine).count())
                .next()
                .unwrap()
        })
        .enumerate()
        .filter(|(_, match_count)| *match_count != 0)
        .for_each(|(index, match_count)| {
            let current_card_count = card_counts[index];
            card_counts[(index + 1)..=(index + match_count)]
                .iter_mut()
                .for_each(|card_count| *card_count += current_card_count);
        });

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = solution(input);
        let expected = 30;
        assert_eq!(result, expected);
    }
}
