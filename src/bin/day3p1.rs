use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day3.txt").expect("Failed to read input");
    let total = compute_sum(&input);
    println!("{}", total);
}

fn compute_sum(schematic: &str) -> i32 {
    let mut sum = 0;
    // Collect our lines into a buffer; these will be our y values
    let lines: Vec<&str> = schematic.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        // Collect our characters; these will be our x values
        let characters: Vec<char> = line.chars().collect();

        let mut min: i32 = 0; // min value to check
        let mut max: i32; // max value to check
        let line_length: i32 = characters.len() as i32;

        for (j, character) in characters.iter().enumerate() {
            if character.is_digit(10) {
                if j == 0 || !characters[j - 1].is_digit(10) {
                    min = j as i32;
                }

                max = j as i32;

                if j == characters.len() - 1 || !characters[j + 1].is_digit(10) {
                    // check current line
                    let mut is_valid = is_adjacent_symbol(lines[i], min, max, line_length);

                    // check line above
                    if i > 0 && !is_valid {
                        is_valid = is_adjacent_symbol(lines[i - 1], min, max, line_length);
                    }

                    // check line below
                    if i + 1 < lines.len() && !is_valid {
                        is_valid = is_adjacent_symbol(lines[i + 1], min, max, line_length);
                    }

                    if is_valid {
                        let slice = &characters[min as usize..=max as usize];
                        sum += parse_number_from_char_array(slice);
                    }
                }
            }
        }
    }

    return sum;
}

fn is_adjacent_symbol(row: &str, min: i32, max: i32, line_length: i32) -> bool {
    for dx in min - 1..=max + 1 {
        if dx > 0 && dx < line_length {
            let nth_char = row.chars().nth(dx as usize).unwrap();
            if nth_char.is_ascii_punctuation() && nth_char != '.' {
                return true;
            }
        }
    }

    return false;
}

fn parse_number_from_char_array(buffer: &[char]) -> i32 {
    let mut number = String::from("");
    for digit in buffer {
        number.push(*digit);
    }

    if let Ok(parsed_number) = number.parse::<i32>() {
        return parsed_number;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_compute_sum() {
        assert_eq!(compute_sum(INPUT), 4361)
    }
}
