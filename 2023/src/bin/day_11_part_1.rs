use std::collections::BTreeSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/day_11");
    let result = solution(input);
    println!("{result}");
}

fn solution(input: &str) -> u32 {
    let row_count = input.lines().count();
    let column_count = input.lines().next().unwrap().chars().count();

    let space_time = input
        .lines()
        .enumerate()
        .flat_map(move |(row_index, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(column_index, char)| match char {
                    '#' => Some(Galaxy(row_index, column_index)),
                    _ => None,
                })
        })
        .fold(
            SpaceTime::big_bang(row_count, column_count),
            |mut space_time, galaxy| {
                space_time.empty_rows.remove(&galaxy.0);
                space_time.empty_columns.remove(&galaxy.1);
                space_time.galaxies.push(galaxy);
                space_time
            },
        );

    space_time
        .galaxies
        .iter()
        .map(|galaxy| {
            Galaxy(
                galaxy.0 + space_time.empty_rows.range(..galaxy.0).count(),
                galaxy.1 + space_time.empty_columns.range(..galaxy.1).count(),
            )
        })
        .tuple_combinations()
        .map(|(a, b)| (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u32)
        .sum()
}

struct SpaceTime {
    empty_rows: BTreeSet<usize>,
    empty_columns: BTreeSet<usize>,
    galaxies: Vec<Galaxy>,
}

impl SpaceTime {
    fn big_bang(row_count: usize, column_count: usize) -> SpaceTime {
        SpaceTime {
            empty_rows: BTreeSet::from_iter(0..row_count),
            empty_columns: BTreeSet::from_iter(0..column_count),
            galaxies: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Galaxy(usize, usize);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = solution(input);
        let expected = 374;
        assert_eq!(result, expected);
    }
}
