use std::fs;

// 12 red cubes, 13 green cubes, and 14 blue cubes
const RED_CUBES: i32 = 12;
const GREEN_CUBES: i32 = 13;
const BLUE_CUBES: i32 = 14;

pub fn main() {
    let input = fs::read_to_string("./inputs/day2.txt").expect("Failed to read input");

    let mut sum: i32 = 0;

    input.split("\n").for_each(|line| {
        let x: Vec<&str> = line.split(':').collect();
        if x.len() > 1 {
            if is_line_valid(x[1]) {
                if let Some(id) = parse_game_id(x[0]) {
                    sum += id;
                }
            }
        }
    });

    println!("{}", sum);
}

fn is_line_valid(line: &str) -> bool {
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

    if max_red > RED_CUBES {
        false
    } else if max_green > GREEN_CUBES {
        false
    } else if max_blue > BLUE_CUBES {
        false
    } else {
        true
    }
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

fn parse_game_id(input: &str) -> Option<i32> {
    let mut iter = input.split_whitespace();

    if let Some(_game_str) = iter.next() {
        if let Some(game_id) = iter.next() {
            if let Ok(n) = game_id.parse::<i32>() {
                return Some(n);
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
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
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
    fn test_is_line_valid() {
        let tests: Vec<&str> = INPUT.split("\n").collect();

        assert_eq!(is_line_valid(tests[0]), true);
        assert_eq!(is_line_valid(tests[1]), true);
        assert_eq!(is_line_valid(tests[2]), false);
        assert_eq!(is_line_valid(tests[3]), false);
        assert_eq!(is_line_valid(tests[4]), true);
    }
}
