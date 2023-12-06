fn main() {
    let input = Record {
        time: 44806572,
        distance: 208158110501102,
    };
    let result = solution(input);
    println!("{result}");
}

fn solution(record: Record) -> u64 {
    record.ways_to_beat()
}

#[derive(Debug, PartialEq, Eq)]
struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    fn ways_to_beat(&self) -> u64 {
        (0..=self.time)
            .filter(|hold_time| {
                let speed = hold_time;
                let travel_time = self.time - hold_time;
                let distance_travelled = speed * travel_time;
                distance_travelled > self.distance
            })
            .count() as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_solution() {
        let input = Record {
            time: 71530,
            distance: 940200,
        };
        let result = solution(input);
        let expected = 71503;
        assert_eq!(result, expected);
    }
}
