use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day3.txt").expect("Failed to read input");
    let part_info = read_data(&input);
    let sum = find_gear_ratio(&input, &part_info);
    println!("{}", sum);
}

struct PartInfo {
    column_index: usize,
    row_index: usize,
    length: usize,
    value: String,
}

fn read_data(schematic: &str) -> Vec<PartInfo> {
    let mut parts: Vec<PartInfo> = Vec::new();

    for (row_index, line) in schematic.lines().enumerate() {
        let mut start_index: Option<usize> = None;
        let mut length: usize = 0;
        let mut value = String::new();

        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if start_index.is_none() {
                    start_index = Some(x);
                }
                length += 1;
                value.push(char);
            } else if start_index.is_some() {
                let start_index_value = start_index.unwrap();
                parts.push(PartInfo {
                    column_index: start_index_value,
                    row_index,
                    length,
                    value: value.clone(),
                });

                start_index = None;
                length = 0;
                value.clear();
            }
        }

        if start_index.is_some() {
            let start_index_value = start_index.unwrap();
            parts.push(PartInfo {
                column_index: start_index_value,
                row_index,
                length,
                value: value.clone(),
            });
        }
    }

    parts
}

fn find_gear_ratio(schematic: &str, part_info: &Vec<PartInfo>) -> i32 {
    let mut sum = 0;

    for (row_index, line) in schematic.lines().enumerate() {
        for (col_index, char) in line.chars().enumerate() {
            if char == '*' {
                let mut adjacent_numbers: Vec<&PartInfo> = Vec::new();
                for part in part_info {
                    if part.row_index > row_index + 1 || part.row_index < row_index - 1 {
                        continue;
                    }

                    if part.column_index == col_index
                        || part.column_index == col_index - 1
                        || part.column_index == col_index + 1
                        || part.column_index + part.length == col_index
                        || part.column_index + part.length == col_index + 1
                    {
                        adjacent_numbers.push(part);
                    }
                }

                if adjacent_numbers.len() == 2 {
                    let v1 = adjacent_numbers[0].value.parse::<i32>().unwrap();
                    let v2 = adjacent_numbers[1].value.parse::<i32>().unwrap();
                    sum += v1 * v2;
                }
            }
        }
    }

    return sum;
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
        ...615....
        ....*.....
        ..575.....
    "};

    #[test]
    fn test_compute_sum() {
        let part_info = read_data(INPUT);
        let sum = find_gear_ratio(INPUT, &part_info);
        assert_eq!(sum, 821460)
    }
}
