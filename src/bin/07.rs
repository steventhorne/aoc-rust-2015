use aoc::_07::{ WireConfiguration, WireOp, Wires, Input };

fn parse(input: &str, overwrite_b: Option<u16>) -> Wires {
    let mut wires = Wires::default();
    input
        .lines()
        .for_each(|l| {
            let mut parts = l.split(" -> ");
            let l = parts.next().unwrap();
            let label = parts.next().unwrap();
            let op_parts = l.split(' ').collect::<Vec<&str>>();
            match op_parts.len() {
                1 => {
                    if label == "b" {
                        if let Some(b) = overwrite_b {
                            return wires.add_wire(WireConfiguration {
                                label: label.to_string(),
                                op: WireOp::Value(Input::Value(b)),
                                value: None,
                            });
                        }
                    }
                    wires.add_wire(WireConfiguration {
                        label: label.to_string(),
                        op: WireOp::Value(Input::from(op_parts[0])),
                        value: None,
                    });
                }
                2 => {
                    wires.add_wire(WireConfiguration {
                        label: label.to_string(),
                        op: WireOp::Not(Input::from(op_parts[1])),
                        value: None,
                    });
                }
                3 => {
                    let op = match op_parts[1] {
                        "AND" => {
                            WireOp::And(Input::from(op_parts[0]), Input::from(op_parts[2]))
                        }
                        "OR" => {
                            WireOp::Or(Input::from(op_parts[0]), Input::from(op_parts[2]))
                        }
                        "LSHIFT" => {
                            WireOp::LShift(Input::from(op_parts[0]), op_parts[2].parse::<u16>().unwrap())
                        }
                        "RSHIFT" => {
                            WireOp::RShift(Input::from(op_parts[0]), op_parts[2].parse::<u16>().unwrap())
                        }
                        _ => panic!("Unknown op: {}", op_parts[1])
                    };
                    wires.add_wire(WireConfiguration { label: label.to_string(), op, value: None });
                }
                _ => panic!("Unknown op: {}", l)
            }
        });

    wires
}

pub fn part_one(input: &str) -> u16 {
    let mut wires = parse(input, None);
    wires.get_and_cache_value("a")
}

pub fn part_two(input: &str) -> u16 {
    let mut wires = parse(input, Some(3176));
    wires.get_and_cache_value("a")
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 7), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), 65079);
    }
}
