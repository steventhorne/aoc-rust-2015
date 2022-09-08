use aoc::string_utils;

use serde_json::Value;

pub fn part_one(input: &str) -> i64 {
    let input = string_utils::strip_newline(input);
    let v: Value = serde_json::from_str(input).unwrap();
    sum_numbers(v, false)
}

pub fn part_two(input: &str) -> i64 {
    let input = string_utils::strip_newline(input);
    let v: Value = serde_json::from_str(input).unwrap();
    sum_numbers(v, true)
}

fn sum_numbers(v: Value, without_red: bool) -> i64 {
    return match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().map(|v| sum_numbers(v.clone(), without_red)).sum(),
        Value::Object(o) => {
            if without_red && o.values().any(|v| v == "red") {
                0
            } else {
                o.values().map(|v| sum_numbers(v.clone(), without_red)).sum()
            }
        }
    }
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 12), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("[1,2,3]"), 6);
        assert_eq!(part_one("[[[3]]]"), 3);
        assert_eq!(part_one(r##"{"a":{"b":4},"c":-1}"##), 3);
        assert_eq!(part_one(r##"{"a":[-1,1]}"##), 0);
        assert_eq!(part_one(r##"[-1,{"a":1}]"##), 0);
        assert_eq!(part_one(r##"[]"##), 0);
        assert_eq!(part_one(r##"{}"##), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(r##"[1,2,3]"##), 6);
        assert_eq!(part_two(r##"[1,{"c":"red","b":2},3]"##), 4);
        assert_eq!(part_two(r##"{"d":"red","e":[1,2,3,4],"f":5}"##), 0);
        assert_eq!(part_two(r##"[1,"red",5]"##), 6);
    }
}
