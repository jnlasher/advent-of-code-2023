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

fn follow_instructions(instructions: &str) -> u32 {
    let mut lines = instructions.lines();

    // First line is our directions
    let directions: Vec<char> = lines.next().unwrap().chars().collect();

    // Skip the empty line
    lines.next();

    // Store the lines as something more useful than raw text
    let mut graph: HashMap<&str, Destination> = HashMap::new();
    for l in lines {
        let mut parts = l.split(" = ");
        let start = parts.next().unwrap();
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

    let mut current_node = "AAA";
    let mut steps = 0;
    let mut next_direction = 0;

    loop {
        if current_node == "ZZZ" {
            break;
        }

        let dest = graph.get(current_node).unwrap();
        let direction = directions[next_direction];

        match direction {
            'R' => current_node = &dest.right,
            'L' => current_node = &dest.left,
            _ => unreachable!(),
        }

        steps += 1;
        next_direction = (next_direction + 1) % directions.len();
    }

    return steps;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const LINEAR_MAP: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const REPEATING_MAP: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test]
    fn test_follow_instructions() {
        let steps = follow_instructions(LINEAR_MAP);
        assert_eq!(steps, 2);
    }

    #[test]
    fn test_repeating_instructions() {
        let steps = follow_instructions(REPEATING_MAP);
        assert_eq!(steps, 6);
    }
}
