use aoc::string_utils;

fn increment_string(input: &mut String) {
    for i in (0..input.len()).rev() {
        let c = input.chars().nth(i).unwrap();
        if c == 'z' {
            input.replace_range(i..i + 1, "a");
        } else {
            input.replace_range(i..i + 1, &((c as u8 + 1) as char).to_string());
            break;
        }
    }
}

fn is_valid_one(input: &str) -> bool {
    let mut has_straight = false;
    let mut straight_count = 0;
    let mut pairs = 0;
    let mut prevent_pair_overlap = false;
    let mut previous: Option<char> = None;

    for c in input.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        if let Some(p) = previous {
            if c == p {
                if !prevent_pair_overlap {
                    pairs += 1;
                    prevent_pair_overlap = true;
                }
            } else {
                prevent_pair_overlap = false;
            }

            if c as u8 == p as u8 + 1 {
                straight_count += 1;
                if straight_count == 2 {
                    has_straight = true;
                }
            } else {
                straight_count = 0;
            }
        }

        previous = Some(c);
    }

    has_straight && pairs == 2
}

pub fn part_one(input: &str) -> String {
    let mut str = string_utils::strip_newline(input).to_string();
    while !is_valid_one(&str) {
        increment_string(&mut str);
    }
    str
}

pub fn part_two(input: &str) -> String {
    let mut str = string_utils::strip_newline(input).to_string();
    while !is_valid_one(&str) {
        increment_string(&mut str);
    }
    increment_string(&mut str);
    while !is_valid_one(&str) {
        increment_string(&mut str);
    }
    str
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 11), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_one(&input), "abcdffaa");
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_two(&input), "");
    }
}
