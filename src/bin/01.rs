pub fn part_one(input: &str) -> i32 {
    let mut floor = 0;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    floor
}

pub fn part_two(input: &str) -> usize {
    part_two_option(input).expect("No basement found")
}

fn part_two_option(input: &str) -> Option<usize> {
    let mut floor = 0;

    for (pos, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor == -1 {
            return Some(pos + 1);
        }
    }

    None
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 1), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), -1);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 5);
    }
}
