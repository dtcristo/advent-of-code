use std::cmp::max;

fn main() {
    let input = include_str!("../../input/day_02");
    let result = solution(input);
    println!("{result}");
}

fn solution(input: &str) -> u32 {
    input
        // Iterate over lines of input.
        .lines()
        // Parse each line into a game.
        .map(Game::from)
        // For each game find the "minimum cubes" and their "power".
        .map(|game| game.minimum_cubes().power())
        // Finally sum these powers.
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    rounds: Vec<Cubes>,
}

#[derive(Debug, PartialEq, Eq)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Game {
    // Take a serialized game and parse it into a `Game` struct.
    fn from(str: &str) -> Self {
        let mut iter = str.split(':');

        let id: u32 = iter
            .next()
            .unwrap()
            .strip_prefix("Game ")
            .unwrap()
            .parse()
            .unwrap();
        let rounds = iter.next().unwrap().split(";").map(Cubes::from).collect();

        Game { id, rounds }
    }
}

impl From<&str> for Cubes {
    // Take a serialized round (eg. " 1 green, 3 red, 6 blue") and parse it
    // into a `Cubes` struct (eg. `Cubes { red: 3, green: 1, blue: 6 }`).
    fn from(str: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        // Split the round on commas into chunks for each count color pair.
        str.split(',').for_each(|count_color| {
            // Split count color pair on whitespace.
            let mut iter = count_color.split_whitespace();
            // First part is always count, parse into `u32`.
            let count: u32 = iter.next().unwrap().parse().unwrap();
            // Next part is always color, based on the color found set
            // count to the appropriate variable.
            match iter.next().unwrap() {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => {}
            }
        });

        Cubes { red, green, blue }
    }
}

impl Game {
    fn minimum_cubes(&self) -> Cubes {
        // Iterate over each round building a new "minumum" cubes set.
        // Start with an empty set and create a new set by by fining
        // the max of each individual color comparing to the current
        // round.
        self.rounds.iter().fold(Cubes::empty(), |acc, round| Cubes {
            red: max(acc.red, round.red),
            green: max(acc.green, round.green),
            blue: max(acc.blue, round.blue),
        })
    }
}

impl Cubes {
    fn empty() -> Self {
        Cubes {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::*;

    #[test]
    fn test_solution() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = solution(input);
        let expected = 2286;
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(" 3 blue, 4 red", Cubes { red: 4, green: 0, blue: 3 })]
    #[case(" 1 red, 2 green", Cubes { red: 1, green: 2, blue: 0 })]
    #[case(" 5 blue, 4 red, 13 green", Cubes { red: 4, green: 13, blue: 5 })]
    fn test_round_from_str(#[case] input: &str, #[case] expected: Cubes) {
        assert_eq!(Cubes::from(input), expected);
    }

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        Game { id: 1, rounds: vec![
            Cubes { red: 4, green: 0, blue: 3 },
            Cubes { red: 1, green: 2, blue: 6 },
            Cubes { red: 0, green: 2, blue: 0 },
        ] }
    )]
    fn test_game_from_str(#[case] input: &str, #[case] expected: Game) {
        assert_eq!(Game::from(input), expected);
    }
}
