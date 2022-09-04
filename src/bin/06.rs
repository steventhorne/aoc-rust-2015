struct Instruction {
    op: InstructionOp,
    start: (u16, u16),
    end: (u16, u16),
}

enum InstructionOp {
    On,
    Off,
    Toggle,
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let mut op = parts.next().unwrap();
        if op == "turn" {
            op = parts.next().unwrap();
        }
        let start = parts.next().unwrap().split(',').map(|x| x.parse::<u16>().unwrap()).collect::<Vec<_>>();
        parts.next(); // skip "through"
        let end = parts.next().unwrap().split(',').map(|x| x.parse::<u16>().unwrap()).collect::<Vec<_>>();

        instructions.push(Instruction {
            op: match op {
                "on" => InstructionOp::On,
                "off" => InstructionOp::Off,
                "toggle" => InstructionOp::Toggle,
                _ => panic!("Unknown op"),
            },
            start: (start[0], start[1]),
            end: (end[0], end[1]),
        });
    }
    instructions
}

pub fn part_one(input: &str) -> usize {
    let instructions = parse(input);
    let mut lights: [bool; 1000 * 1000] = [false; 1000 * 1000];
    for instruction in instructions {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                let key: usize = x as usize + y as usize * 1000;
                lights[key] = match instruction.op {
                    InstructionOp::On => true,
                    InstructionOp::Off => false,
                    InstructionOp::Toggle => !lights[key],
                };
            }
        }
    }
    lights.iter().filter(|x| **x).count()
}

pub fn part_two(input: &str) -> u32 {
    let instructions = parse(input);
    let mut lights: Vec<u32> = vec![0; 1000 * 1000];
    for instruction in instructions {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                let key: usize = x as usize + y as usize * 1000;
                lights[key] = match instruction.op {
                    InstructionOp::On => lights[key] + 1,
                    InstructionOp::Off => lights[key].saturating_sub(1),
                    InstructionOp::Toggle => lights[key] + 2,
                };
            }
        }
    }
    lights.iter().sum()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 6), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), 998996);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), 1001996);
    }
}
