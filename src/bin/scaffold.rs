use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'd', long = "day")]
    day: u8,

    #[clap(short = 'i', long = "input")]
    input: bool,
}

fn main() {
    let args = Args::parse();
    let day = format!("{:02}", args.day);

    if args.input {
        File::create(format!("src/inputs/{}.txt", day)).expect("Failed to create input file");
        println!("Created input file for day {}", day);
    }

    File::create(format!("src/examples/{}.txt", day)).expect("Failed to create example file");
    println!("Created example file for day {}", day);

    let mut file =
        File::create(format!("src/bin/{}.rs", day)).expect("Failed to create module file");

    let module_contents = r###"pub fn part_one(input: &str) -> u32 {
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", DAY), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", DAY);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", DAY);
        assert_eq!(part_two(&input), 0);
    }
}
"###;
    file.write_all(
        module_contents
            .replace("DAY", &(&args.day).to_string())
            .as_bytes(),
    )
    .expect("Failed to write to input file");

    println!("Created module file for day {}", day);
}
