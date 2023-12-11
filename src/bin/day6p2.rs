use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day6.txt").expect("Failed to read input");
    let race_data = read_race_data(&input);
    let error_margin = compute_error_margin(race_data);
    println!("Error margin: {}", error_margin);
}

#[derive(Debug)]
struct RaceRecord {
    time: u64,
    distance: u64,
}

impl RaceRecord {
    fn new(time: u64, distance: u64) -> RaceRecord {
        RaceRecord { time, distance }
    }

    fn get_winning_distances(&self) -> Vec<u64> {
        let mut all_distances: Vec<u64> = Vec::new();
        for duration in 0..=self.time {
            let time_left = self.time - duration;
            let total_distance = duration * time_left;
            all_distances.push(total_distance);
        }
        all_distances
            .into_iter()
            .filter(|d| *d > self.distance)
            .collect()
    }
}

fn read_race_data(input: &str) -> RaceRecord {
    let lines: Vec<&str> = input.lines().collect();
    assert!(lines.len() == 2);

    let numbers: String = lines[0].split_whitespace().skip(1).collect();
    let time: u64 = numbers.parse().unwrap();

    let numbers: String = lines[1].split_whitespace().skip(1).collect();
    let distance = numbers.parse().unwrap();

    RaceRecord::new(time, distance)
}

fn compute_error_margin(record: RaceRecord) -> u64 {
    record.get_winning_distances().len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_read_race_data() {
        let race_data = read_race_data(INPUT);
        assert_eq!(race_data.time, 71530);
        assert_eq!(race_data.distance, 940200);
    }

    #[test]
    fn test_travel_distance() {
        let race_data: RaceRecord = RaceRecord {
            time: 7,
            distance: 9,
        };
        let distances = vec![0, 6, 10, 12, 12, 10, 6, 0];
        assert_eq!(race_data.get_winning_distances(), distances);
    }

    #[test]
    fn test_compute_error_margin() {
        // let race_data = read_race_data(INPUT);
        // assert_eq!(compute_error_margin(race_data), 288);
    }
}
