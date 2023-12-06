fn main() {
    let input = vec![
        Record {
            time: 44,
            distance: 208,
        },
        Record {
            time: 80,
            distance: 1581,
        },
        Record {
            time: 65,
            distance: 1050,
        },
        Record {
            time: 72,
            distance: 1102,
        },
    ];
    let result = solution(input);
    println!("{result}");
}

fn solution(records: Vec<Record>) -> u32 {
    records.iter().map(Record::ways_to_beat).product()
}

#[derive(Debug, PartialEq, Eq)]
struct Record {
    time: u32,
    distance: u32,
}

impl Record {
    fn ways_to_beat(&self) -> u32 {
        (0..=self.time)
            .filter(|hold_time| {
                let speed = hold_time;
                let travel_time = self.time - hold_time;
                let distance_travelled = speed * travel_time;
                distance_travelled > self.distance
            })
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = vec![
            Record {
                time: 7,
                distance: 9,
            },
            Record {
                time: 15,
                distance: 40,
            },
            Record {
                time: 30,
                distance: 200,
            },
        ];
        let result = solution(input);
        let expected = 288;
        assert_eq!(result, expected);
    }
}
