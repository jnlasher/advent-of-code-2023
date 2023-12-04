use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day3.txt").expect("Failed to read input");

    let mut sum: i32 = 0;

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
}
