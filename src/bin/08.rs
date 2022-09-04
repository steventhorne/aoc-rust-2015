pub fn part_one(input: &str) -> usize {
    let a_len: usize = input
        .lines()
        .map(|l| l.len())
        .sum();

    let p_len: usize = input
        .lines()
        .map(parsed_len)
        .sum();

    a_len - p_len
}

fn parsed_len(input: &str) -> usize {
    let mut escaping = false;
    let mut escaped = 0;
    let mut unicode = 0;
    input
        .chars()
        .for_each(|c| {
            if escaping {
                if c == 'x' {
                    unicode += 1;
                } else {
                    escaped += 1;
                }
                escaping = false;
            }
            else if c == '\\' {
                escaping = true;
            }
        });
    input.len() - 2 - escaped - (unicode * 3)
}

pub fn part_two(input: &str) -> usize {
    let a_len: usize = input
        .lines()
        .map(|l| l.len())
        .sum();

    let e_len: usize = input
        .lines()
        .map(escaped_len)
        .sum();

    e_len - a_len
}

fn escaped_len(input: &str) -> usize {
    let mut escaped = 0;
    input
        .chars()
        .for_each(|c| {
            if c == '"' || c == '\\' {
                escaped += 1;
            }
        });
    input.len() + 2 + escaped
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 8), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_one(&input), 12);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_two(&input), 19);
    }
}
