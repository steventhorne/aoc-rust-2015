use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut houses = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    houses.insert((0, 0));

    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Unexpected character: {}", c),
        }
        if houses.get(&(x, y)).is_none() {
            houses.insert((x, y));
        }
    }

    houses.len() as usize
}

pub fn part_two(input: &str) -> usize {
    let mut houses = HashSet::new();

    let mut santas_turn = true;

    let mut sx = 0;
    let mut sy = 0;
    let mut bx = 0;
    let mut by = 0;

    houses.insert((0, 0));

    for c in input.chars() {
        match c {
            '^' => {
                if santas_turn {
                    sy += 1
                } else {
                    by += 1
                }
            }
            'v' => {
                if santas_turn {
                    sy -= 1
                } else {
                    by -= 1
                }
            }
            '>' => {
                if santas_turn {
                    sx += 1
                } else {
                    bx += 1
                }
            }
            '<' => {
                if santas_turn {
                    sx -= 1
                } else {
                    bx -= 1
                }
            }
            _ => panic!("Unexpected character: {}", c),
        }

        if santas_turn {
            if houses.get(&(sx, sy)).is_none() {
                houses.insert((sx, sy));
            }
        } else if houses.get(&(bx, by)).is_none() {
            houses.insert((bx, by));
        }

        santas_turn = !santas_turn;
    }

    houses.len() as usize
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 3), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), 4);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), 3);
    }
}
