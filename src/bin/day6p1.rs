use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day6.txt").expect("Failed to read input");
    let race_data = read_race_data(&input);
    let error_margin = compute_error_margin(race_data);
    println!("Error margin: {}", error_margin);
}

#[derive(Debug)]
struct RaceRecord {
    time: u32,
    distance: u32,
}

impl RaceRecord {
    fn new(time: u32, distance: u32) -> RaceRecord {
        RaceRecord { time, distance }
    }

    fn get_winning_distances(&self) -> Vec<u32> {
        let mut all_distances: Vec<u32> = Vec::new();
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

fn read_race_data(input: &str) -> Vec<RaceRecord> {
    let lines: Vec<&str> = input.lines().collect();
    assert!(lines.len() == 2);

    let all_times: Vec<u32> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let all_distances: Vec<u32> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    assert!(all_times.len() == all_distances.len());

    all_times
        .iter()
        .enumerate()
        .map(|(i, time)| {
            let distance = all_distances[i];
            RaceRecord::new(*time, distance)
        })
        .collect()
}

fn compute_error_margin(records: Vec<RaceRecord>) -> u32 {
    records
        .into_iter()
        .map(|record| record.get_winning_distances().len() as u32)
        .reduce(|acc, el| acc * el)
        .unwrap()
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
        assert_eq!(race_data.len(), 3);
        assert_eq!(race_data[0].time, 7);
        assert_eq!(race_data[0].distance, 9);
        assert_eq!(race_data[1].time, 15);
        assert_eq!(race_data[1].distance, 40);
        assert_eq!(race_data[2].time, 30);
        assert_eq!(race_data[2].distance, 200);
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
        let race_data = read_race_data(INPUT);
        assert_eq!(compute_error_margin(race_data), 288);
    }
}
