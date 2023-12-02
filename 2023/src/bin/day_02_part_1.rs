use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/day_02");
    let result = solution(input);

    println!("{result}");

    Ok(())
}

fn solution(input: &str) -> u32 {
    input
        // Iterate over lines of input.
        .lines()
        // Parse each line into a game.
        .map(Game::from)
        // Filter only "possible" games.
        .filter(|game| game.is_possible())
        // Get the IDs of these games.
        .map(|game| game.id)
        // Finally sum these IDs.
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
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
        let rounds = iter.next().unwrap().split(";").map(Round::from).collect();

        Game { id, rounds }
    }
}

impl From<&str> for Round {
    // Take a serialized round (eg. " 1 green, 3 red, 6 blue") and parse it
    // into a `Round` struct (eg. `Round { red: 3, green: 1, blue: 6 }`).
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

        Round { red, green, blue }
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|round| round.is_possible())
    }
}

impl Round {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[test]
    fn test_solution() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = solution(input);
        let expected = 8;
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(Round { red: 4, green: 0, blue: 3 }, true)]
    #[case(Round { red: 20, green: 8, blue: 6 }, false)]
    fn test_round_is_possible(#[case] round: Round, #[case] expected: bool) {
        assert_eq!(round.is_possible(), expected);
    }

    #[rstest]
    #[case(
        Game { id: 1, rounds: vec![
            Round { red: 4, green: 0, blue: 3 },
            Round { red: 1, green: 2, blue: 6 }
        ] },
        true
    )]
    #[case(
        Game { id: 2, rounds: vec![
            Round { red: 20, green: 8, blue: 6 },
            Round { red: 1, green: 2, blue: 6 }
        ] },
        false
    )]
    fn test_game_is_possible(#[case] game: Game, #[case] expected: bool) {
        assert_eq!(game.is_possible(), expected);
    }

    #[rstest]
    #[case(" 3 blue, 4 red", Round { red: 4, green: 0, blue: 3 })]
    #[case(" 1 red, 2 green", Round { red: 1, green: 2, blue: 0 })]
    #[case(" 5 blue, 4 red, 13 green", Round { red: 4, green: 13, blue: 5 })]
    fn test_round_from_str(#[case] input: &str, #[case] expected: Round) {
        assert_eq!(Round::from(input), expected);
    }

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        Game { id: 1, rounds: vec![
            Round { red: 4, green: 0, blue: 3 },
            Round { red: 1, green: 2, blue: 6 },
            Round { red: 0, green: 2, blue: 0 },
        ] }
    )]
    fn test_game_from_str(#[case] input: &str, #[case] expected: Game) {
        assert_eq!(Game::from(input), expected);
    }
}