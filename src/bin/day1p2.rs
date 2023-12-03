use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day1.txt").expect("Failed to read input");
    let lines: Vec<&str> = input.split("\n").collect();
    let result = traverse_lines(lines);
    println!("{}", result);
}

fn traverse_lines(lines: Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    for line in lines.iter() {
        sum += join_first_last(line);
    }
    return sum;
}

fn join_first_last(line: &str) -> i32 {
    // Keep the first and last letters in the event we have shared letters,
    // for example, "eightwo" should be 82
    let cooked_line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let digit_characters: Vec<char> = cooked_line.chars().filter(|c| c.is_digit(10)).collect();
    let num: i32 = format!(
        "{}{}",
        digit_characters.first().unwrap_or(&'0'),
        digit_characters.last().unwrap_or(&'0')
    )
    .parse()
    .unwrap();

    return num;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_join_first_last() {
        let tests: Vec<&str> = INPUT.split("\n").collect();

        let mut result = join_first_last(tests[0]);
        assert_eq!(result, 29);

        result = join_first_last(tests[1]);
        assert_eq!(result, 83);

        result = join_first_last(tests[2]);
        assert_eq!(result, 13);

        result = join_first_last(tests[3]);
        assert_eq!(result, 24);

        result = join_first_last(tests[4]);
        assert_eq!(result, 42);

        result = join_first_last(tests[5]);
        assert_eq!(result, 14);

        result = join_first_last(tests[6]);
        assert_eq!(result, 76);
    }

    #[test]
    fn test_traverse_lines() {
        let result = traverse_lines(INPUT.split("\n").collect());
        assert_eq!(result, 281);
    }
}
