struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

fn parse_input(input: &str) -> Vec<Dimensions> {
    input.lines().map(|line| {
        let mut parts = line.split('x');
        let length = parts.next().unwrap().parse::<u32>().unwrap();
        let width = parts.next().unwrap().parse::<u32>().unwrap();
        let height = parts.next().unwrap().parse::<u32>().unwrap();
        Dimensions { length, width, height }
    }).collect()
}

pub fn part_one(input: &str) -> u32 {
		let dimensions = parse_input(input);
    dimensions.iter().map(get_area).sum()
}

fn get_area(dim: &Dimensions) -> u32 {
    let x = dim.length * dim.width;
    let y = dim.width * dim.height;
    let z = dim.height * dim.length;
    let smallest = *[x, y, z].iter().min().unwrap();

    (2 * x) + (2 * y) + (2 * z) + smallest
}

pub fn part_two(input: &str) -> u32 {
    let dimensions = parse_input(input);
    dimensions.iter().map(get_ribbon_length).sum()
}

fn get_ribbon_length(dim: &Dimensions) -> u32 {
    let x = dim.length + dim.width;
    let y = dim.width + dim.height;
    let z = dim.height + dim.length;

    let smallest_perimiter = *[x, y, z].iter().min().unwrap() * 2;
    let volume = dim.length * dim.width * dim.height;

    smallest_perimiter + volume
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 2), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 101);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 48);
    }
}
