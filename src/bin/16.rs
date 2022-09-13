use std::cmp::Ordering;

struct Aunt {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Aunt {
    fn from(input: &str) -> Aunt {
        let parts: Vec<&str> = input.split(' ').collect();

        let mut children: Option<u32> = None;
        let mut cats: Option<u32> = None;
        let mut samoyeds: Option<u32> = None;
        let mut pomeranians: Option<u32> = None;
        let mut akitas: Option<u32> = None;
        let mut vizslas: Option<u32> = None;
        let mut goldfish: Option<u32> = None;
        let mut trees: Option<u32> = None;
        let mut cars: Option<u32> = None;
        let mut perfumes: Option<u32> = None;

        let mut base = 2;
        for _ in 0..3 {
            let name = parts[base].trim_end_matches(':');
            let amount = parts[base + 1]
                .trim_end_matches(',')
                .parse::<u32>()
                .unwrap();
            match name {
                "children" => children = Some(amount),
                "cats" => cats = Some(amount),
                "samoyeds" => samoyeds = Some(amount),
                "pomeranians" => pomeranians = Some(amount),
                "akitas" => akitas = Some(amount),
                "vizslas" => vizslas = Some(amount),
                "goldfish" => goldfish = Some(amount),
                "trees" => trees = Some(amount),
                "cars" => cars = Some(amount),
                "perfumes" => perfumes = Some(amount),
                _ => {}
            }
            base += 2;
        }

        Aunt {
            number: parts[1].trim_end_matches(':').parse::<u32>().unwrap(),
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        }
    }
}

fn parse(input: &str) -> Vec<Aunt> {
    let mut aunts = vec![];
    for line in input.trim_end_matches('\n').lines() {
        aunts.push(Aunt::from(line))
    }
    aunts
}

fn check_value(param: &Option<u32>, ord: Ordering, value: u32) -> bool {
    if let Some(v) = param {
        return (*v).cmp(&value) == ord;
    }
    true
}

pub fn part_one(input: &str) -> u32 {
    let aunts = parse(input);
    for aunt in aunts.iter() {
        if !check_value(&aunt.children, Ordering::Equal, 3)
            || !check_value(&aunt.cats, Ordering::Equal, 7)
            || !check_value(&aunt.samoyeds, Ordering::Equal, 2)
            || !check_value(&aunt.pomeranians, Ordering::Equal, 3)
            || !check_value(&aunt.akitas, Ordering::Equal, 0)
            || !check_value(&aunt.vizslas, Ordering::Equal, 0)
            || !check_value(&aunt.goldfish, Ordering::Equal, 5)
            || !check_value(&aunt.trees, Ordering::Equal, 3)
            || !check_value(&aunt.cars, Ordering::Equal, 2)
            || !check_value(&aunt.perfumes, Ordering::Equal, 1)
        {
            continue;
        }
        return aunt.number;
    }
    0
}

pub fn part_two(input: &str) -> u32 {
    let aunts = parse(input);
    for aunt in aunts.iter() {
        if !check_value(&aunt.children, Ordering::Equal, 3)
            || !check_value(&aunt.cats, Ordering::Greater, 7)
            || !check_value(&aunt.samoyeds, Ordering::Equal, 2)
            || !check_value(&aunt.pomeranians, Ordering::Less, 3)
            || !check_value(&aunt.akitas, Ordering::Equal, 0)
            || !check_value(&aunt.vizslas, Ordering::Equal, 0)
            || !check_value(&aunt.goldfish, Ordering::Less, 5)
            || !check_value(&aunt.trees, Ordering::Greater, 3)
            || !check_value(&aunt.cars, Ordering::Equal, 2)
            || !check_value(&aunt.perfumes, Ordering::Equal, 1)
        {
            continue;
        }
        return aunt.number;
    }
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 16), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 16);
        let aunts = parse(&input);
        assert_eq!(aunts[0].number, 1);
        assert!(aunts[0].children.is_none());
        assert!(aunts[0].cats.is_none());
        assert!(aunts[0].samoyeds.is_none());
        assert!(aunts[0].pomeranians.is_none());
        assert_eq!(aunts[0].akitas.unwrap(), 0);
        assert!(aunts[0].vizslas.is_none());
        assert_eq!(aunts[0].goldfish.unwrap(), 6);
        assert_eq!(aunts[0].trees.unwrap(), 9);
        assert!(aunts[0].cars.is_none());
        assert!(aunts[0].perfumes.is_none());

        assert_eq!(aunts[1].number, 2);
        assert!(aunts[1].children.is_none());
        assert!(aunts[1].cats.is_none());
        assert!(aunts[1].samoyeds.is_none());
        assert!(aunts[1].pomeranians.is_none());
        assert_eq!(aunts[1].akitas.unwrap(), 0);
        assert!(aunts[1].vizslas.is_none());
        assert_eq!(aunts[1].goldfish.unwrap(), 7);
        assert_eq!(aunts[1].trees.unwrap(), 1);
        assert!(aunts[1].cars.is_none());
        assert!(aunts[1].perfumes.is_none());
    }
}
