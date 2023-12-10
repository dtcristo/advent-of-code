use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/day_09");
    let result = solution(input);
    println!("{result}");
}

fn solution(input: &str) -> i32 {
    // For each line of input.
    input
        .lines()
        // Split whitespace and parse each value into `i32`.
        .map(|line| {
            line.split(' ')
                .map(|value| value.parse().unwrap())
                // Reverse values to extrapolate backwards.
                // This line is the only change from part 1.
                .rev()
                .collect()
        })
        // Extrapolate the values.
        .map(|values| extrapolate(values))
        // Sum extrapolations.
        .sum()
}

// Recursively extrapolate a set of values.
fn extrapolate(values: Vec<i32>) -> i32 {
    // Base case when all values are 0, extrapolation is 0.
    if values.iter().all(|&value| value == 0) {
        0
    } else {
        // Child values are calculated by the difference of each value pair.
        let children = values.iter().tuple_windows().map(|(a, b)| b - a).collect();
        // Add the last value to the recursive extrapolation of the children
        // to calucate this extrapolated value.
        values.last().unwrap() + extrapolate(children)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = solution(input);
        let expected = 2;
        assert_eq!(result, expected);
    }
}
