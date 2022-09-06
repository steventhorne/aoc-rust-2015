use aoc::string_utils;

fn look_and_say(input: &str) -> String {
    let mut result = String::new();
    let mut chars = string_utils::strip_newline(input).chars();
    let mut current = chars.next().unwrap();
    let mut count = 1;
    for c in chars {
        if c == current {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(current);
            current = c;
            count = 1;
        }
    }
    result.push_str(&count.to_string());
    result.push(current);
    result
}

pub fn part_one(input: &str) -> usize {
    let mut input = input.to_string();
    for _ in 0..40 {
        input = look_and_say(&input);
    }
    input.len()
}

pub fn part_two(input: &str) -> usize {
    let mut input = input.to_string();
    for _ in 0..50 {
        input = look_and_say(&input);
    }
    input.len()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 10), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(look_and_say("1"), "11");
    }

    #[test]
    fn test_two() {
        assert_eq!(look_and_say("11"), "21");
    }

    #[test]
    fn test_three() {
        assert_eq!(look_and_say("21"), "1211");
    }

    #[test]
    fn test_four() {
        assert_eq!(look_and_say("1211"), "111221");
    }

    #[test]
    fn test_five() {
        assert_eq!(look_and_say("111221"), "312211");
    }
}

