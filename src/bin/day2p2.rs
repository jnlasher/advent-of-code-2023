use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day2.txt").expect("Failed to read input");

    let mut sum: i32 = 0;

    input.split("\n").for_each(|line| {
        let x: Vec<&str> = line.split(':').collect();
        sum += compute_power(x[1]);
    });

    println!("{}", sum);
}

fn compute_power(line: &str) -> i32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let draws = line.split(';');
    for draw in draws {
        let rolls = draw.split(',');
        for roll in rolls {
            if let Some((number, color)) = parse_number_and_color(roll) {
                match color {
                    "red" => {
                        if number > max_red {
                            max_red = number;
                        }
                    }
                    "green" => {
                        if number > max_green {
                            max_green = number;
                        }
                    }
                    "blue" => {
                        if number > max_blue {
                            max_blue = number;
                        }
                    }
                    _ => println!("What color did you pass in?"),
                }
            } else {
                println!("Something went wrong parsing the roll");
            }
        }
    }

    max_red * max_green * max_blue
}

fn parse_number_and_color(input: &str) -> Option<(i32, &str)> {
    let mut iter = input.split_whitespace();

    if let Some(number_str) = iter.next() {
        if let Ok(number) = number_str.parse::<i32>() {
            if let Some(color) = iter.next() {
                return Some((number, color));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_parse_number_and_color() {
        let mut r1 = parse_number_and_color("3 blue").unwrap();
        assert_eq!(r1.0, 3);
        assert_eq!(r1.1, "blue");

        r1 = parse_number_and_color("13 green").unwrap();
        assert_eq!(r1.0, 13);
        assert_eq!(r1.1, "green");

        r1 = parse_number_and_color("1 red").unwrap();
        assert_eq!(r1.0, 1);
        assert_eq!(r1.1, "red");
    }

    #[test]
    fn test_compute_power() {
        let tests: Vec<&str> = INPUT.split("\n").collect();

        assert_eq!(compute_power(tests[0]), 48);
        assert_eq!(compute_power(tests[1]), 12);
        assert_eq!(compute_power(tests[2]), 1560);
        assert_eq!(compute_power(tests[3]), 630);
        assert_eq!(compute_power(tests[4]), 36);
    }
}
