pub fn part_one(input: &str) -> usize {
    input.lines().filter(|l| is_nice_one(l)).count()
}

fn is_nice_one(input: &str) -> bool {
    let mut vowels = 0;
    let mut double = false;
    let mut previous: Option<char> = None;
    for c in input.chars() {
        if let Some(p) = previous {
            if p == 'a' && c == 'b'
                || p == 'c' && c == 'd'
                || p == 'p' && c == 'q'
                || p == 'x' && c == 'y'
            {
                return false;
            }

            if c == p {
                double = true;
            }
        }

        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowels += 1;
        }

        previous = Some(c);
    }

    vowels >= 3 && double
}

pub fn part_two(input: &str) -> usize {
    input.lines().filter(|l| is_nice_two(l)).count()
}

fn is_nice_two(input: &str) -> bool {
    let mut previous: Option<char> = None;
    let mut two_previous: Option<char> = None;

    let mut double_repeat: bool = false;
    let mut split_repeat: bool = false;

    for (i, c) in input.chars().enumerate() {
        if let Some(p) = two_previous {
            if p == c {
                split_repeat = true;
            }
        }

        if let Some(p) = previous {
            if i > 1 && input[..i - 1].contains(&String::from_iter(vec![p, c])) {
                double_repeat = true;
            }

            two_previous = Some(p);
        }

        previous = Some(c);
    }

    double_repeat && split_repeat
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 5), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), 2);
    }
}
