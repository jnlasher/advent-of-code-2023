use std::collections::HashMap;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day8.txt").expect("Failed to read input");
    let steps = follow_instructions(&input);
    println!("Finished map in {} steps", steps);
}

#[derive(Debug)]
struct Destination {
    left: String,
    right: String,
}

fn follow_instructions(instructions: &str) -> u64 {
    let mut lines = instructions.lines();

    // First line is our directions
    let directions: Vec<char> = lines.next().unwrap().chars().collect();

    // Skip the empty line
    lines.next();

    // Store the lines as something more useful than raw text
    let mut graph: HashMap<&str, Destination> = HashMap::new();
    let mut starting_locations: Vec<&str> = vec![];

    for l in lines {
        let mut parts = l.split(" = ");

        let start = parts.next().unwrap();
        if start.ends_with('A') {
            starting_locations.push(start);
        }

        let mut destinations = parts.next().unwrap().split(", ");
        let left = destinations.next().unwrap().trim_start_matches("(");
        let right = destinations.next().unwrap().trim_end_matches(")");

        graph.insert(
            start,
            Destination {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }

    let n_periods: Vec<u64> = starting_locations
        .iter()
        .map(|loc| {
            let mut steps: u64 = 0;
            let mut next_direction = 0;
            let mut current_node = *loc;

            loop {
                if current_node.ends_with('Z') {
                    return steps;
                }

                let dest = graph.get(current_node).unwrap();
                let direction = directions[next_direction];
                current_node = match direction {
                    'R' => &dest.right,
                    'L' => &dest.left,
                    _ => panic!("Reached value with: {:?}", direction),
                };

                steps += 1;
                next_direction = (next_direction + 1) % directions.len();
            }
        })
        .collect();

    n_periods
        .into_iter()
        .reduce(|acc, e| num::integer::lcm(acc, e))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn test_follow_instructions() {
        let steps = follow_instructions(INPUT);
        assert_eq!(steps, 6);
    }
}
