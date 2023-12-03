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
    let digit_characters: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
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
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn test_join_first_last() {
        let mut result = join_first_last("1abc2");
        assert_eq!(result, 12);

        result = join_first_last("pqr3stu8vwx");
        assert_eq!(result, 38);

        result = join_first_last("a1b2c3d4e5f");
        assert_eq!(result, 15);

        result = join_first_last("treb7uchet");
        assert_eq!(result, 77);
    }

    #[test]
    fn test_traverse_lines() {
        let result = traverse_lines(INPUT.split("\n").collect());
        assert_eq!(result, 142);
    }
}
