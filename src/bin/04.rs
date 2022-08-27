#[macro_use(concat_string)]
extern crate concat_string;
use md5::Digest;

fn parse_input(input: &str) -> &str {
    input.strip_suffix('\n').unwrap()
}

fn has_zeroes(input: &Digest, num_zeroes: usize) -> bool {
    input.iter().take(num_zeroes / 2).all(|b| *b == 0)
        && (num_zeroes % 2 == 0 || input.iter().nth(num_zeroes / 2).unwrap() < &0x10)
}

pub fn part_one(input: &str) -> usize {
    let input = parse_input(input);
    let mut num = 0;
    loop {
        let result = md5::compute(concat_string!(input, &num.to_string()));
        if has_zeroes(&result, 5) {
            return num;
        }
        num += 1;
    }
}

pub fn part_two(input: &str) -> usize {
    let input = parse_input(input);
    let mut num = 0;
    loop {
        let result = md5::compute(concat_string!(input, &num.to_string()));
        if has_zeroes(&result, 6) {
            return num;
        }
        num += 1;
    }
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 4), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), 609043);
    }
}
