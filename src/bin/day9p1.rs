use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day9.txt").expect("Failed to read input");
    let report = parse_lines(&input);
    let sum = report.iter().fold(0, |acc, e| acc + predict(e));
    println!("Sum: {}", sum);
}

fn predict(history: &Vec<i32>) -> i32 {
    let mut predictor: i32 = *history.last().unwrap();
    let mut diff = differences(history);

    loop {
        if diff.iter().all(|e| *e == 0) {
            break;
        }
        predictor += diff.last().unwrap();
        diff = differences(&diff);
    }

    predictor
}

fn parse_lines(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn differences(input: &Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();

    for i in 1..input.len() {
        output.push(input[i] - input[i - 1]);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differences() {
        let diff = differences(&vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(diff.len(), 5);
        assert_eq!(diff[0], 3);
        assert_eq!(diff[1], 3);
        assert_eq!(diff[2], 3);
        assert_eq!(diff[3], 3);
        assert_eq!(diff[4], 3);

        let diff = differences(&vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(diff.len(), 5);
        assert_eq!(diff[0], 2);
        assert_eq!(diff[1], 3);
        assert_eq!(diff[2], 4);
        assert_eq!(diff[3], 5);
        assert_eq!(diff[4], 6);

        let diff = differences(&vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(diff.len(), 5);
        assert_eq!(diff[0], 3);
        assert_eq!(diff[1], 3);
        assert_eq!(diff[2], 5);
        assert_eq!(diff[3], 9);
        assert_eq!(diff[4], 15);
    }

    #[test]
    fn test_predict() {
        assert_eq!(predict(&vec![10, 13, 16, 21, 30, 45]), 68);
    }
}
