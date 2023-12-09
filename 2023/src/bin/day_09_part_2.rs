use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/day_09");
    let result = solution(input);
    println!("{result}");
}

fn solution(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|value| value.parse::<i32>().unwrap())
                .rev()
                .collect()
        })
        .map(|values| extrapolate(values))
        .sum()
}

fn extrapolate(values: Vec<i32>) -> i32 {
    if values.iter().all(|&value| value == 0) {
        return 0;
    } else {
        let children = values.iter().tuple_windows().map(|(a, b)| b - a).collect();
        return values.last().unwrap() + extrapolate(children);
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
