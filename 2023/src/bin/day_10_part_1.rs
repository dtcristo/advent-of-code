use std::iter;

fn main() {
    let input = include_str!("../../input/day_10");
    let result = solution(input);
    println!("{result}");
}

fn solution(input: &str) -> u32 {
    let tiles = Tiles::parse(input);
    let start_direction = *tiles.start_connections().first().unwrap();
    let start_location = tiles.start_location.unwrap();
    let next_location = start_location.translate(start_direction).unwrap();

    iter::successors(
        Some((start_direction, next_location)),
        |&(direction, location)| {
            if location == start_location {
                None
            } else {
                Some(tiles.get(location).unwrap().traverse(direction, location))
            }
        },
    )
    .count() as u32
        / 2
}

struct Tiles {
    grid: Vec<Vec<Tile>>,
    start_location: Option<Location>,
}

impl Tiles {
    fn parse(input: &str) -> Tiles {
        let mut start_location = None;
        let grid: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(column_index, char)| {
                        let tile = Tile::parse(char);
                        if tile == Tile::Start {
                            start_location = Some(Location(row_index, column_index));
                        }
                        tile
                    })
                    .collect()
            })
            .collect();

        Tiles {
            grid,
            start_location,
        }
    }

    fn get(&self, location: Location) -> Option<Tile> {
        self.grid
            .get(location.0)
            .and_then(|row| row.get(location.1).copied())
    }

    // Determine the directions the start location is connected to.
    fn start_connections(&self) -> Vec<Direction> {
        Direction::iter()
            .filter_map(|direction| {
                self.start_location
                    .unwrap()
                    .translate(direction)
                    .and_then(|adjacent_location| {
                        self.get(adjacent_location).and_then(|adjacent_tile| {
                            if adjacent_tile.is_connected(direction.inverse()) {
                                Some(direction)
                            } else {
                                None
                            }
                        })
                    })
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Location(usize, usize);

impl Location {
    fn translate(&self, direction: Direction) -> Option<Location> {
        let mut location = self.clone();
        match direction {
            Direction::North => {
                if location.0 == 0 {
                    return None;
                } else {
                    location.0 -= 1
                }
            }
            Direction::South => location.0 += 1,
            Direction::East => location.1 += 1,
            Direction::West => {
                if location.1 == 0 {
                    return None;
                } else {
                    location.1 -= 1
                }
            }
        }

        Some(location)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        .copied()
    }

    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

impl Tile {
    fn parse(input: char) -> Tile {
        match input {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEastBend,
            'J' => Tile::NorthWestBend,
            '7' => Tile::SouthWestBend,
            'F' => Tile::SouthEastBend,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!(),
        }
    }

    fn connections(&self) -> [Direction; 2] {
        match self {
            Tile::Vertical => [Direction::North, Direction::South],
            Tile::Horizontal => [Direction::East, Direction::West],
            Tile::NorthEastBend => [Direction::North, Direction::East],
            Tile::NorthWestBend => [Direction::North, Direction::West],
            Tile::SouthWestBend => [Direction::South, Direction::West],
            Tile::SouthEastBend => [Direction::South, Direction::East],
            _ => panic!(),
        }
    }

    fn is_connected(&self, direction: Direction) -> bool {
        *self != Tile::Ground && self.connections().into_iter().any(|d| d == direction)
    }

    fn traverse(&self, direction: Direction, location: Location) -> (Direction, Location) {
        let next_direction = self
            .connections()
            .into_iter()
            .find(|&d| d != direction.inverse())
            .unwrap();
        let next_location = location.translate(next_direction).unwrap();

        (next_direction, next_location)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        4
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        8
    )]
    fn test_solution(#[case] input: &str, #[case] expected: u32) {
        let result = solution(input);
        assert_eq!(result, expected);
    }
}
