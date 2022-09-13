use std::cmp::Ordering;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse(input: &str) -> Vec<Ingredient> {
    let mut ingredients = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        ingredients.push(Ingredient {
            capacity: parts[2].trim_end_matches(',').parse::<i32>().unwrap(),
            durability: parts[4].trim_end_matches(',').parse::<i32>().unwrap(),
            flavor: parts[6].trim_end_matches(',').parse::<i32>().unwrap(),
            texture: parts[8].trim_end_matches(',').parse::<i32>().unwrap(),
            calories: parts[10].trim_end_matches(',').parse::<i32>().unwrap(),
        });
    }
    ingredients
}

fn find_score_one(ingredients: &[Ingredient]) -> i32 {
    let mut best = i32::MIN;

    for tsp_one in 0..101 {
        for tsp_two in 0..(101 - tsp_one) {
            for tsp_three in 0..(101 - tsp_one - tsp_two) {
                let tsp_four = 100 - tsp_one - tsp_two - tsp_three;
                let capacity = ingredients[0].capacity * tsp_one
                    + ingredients[1].capacity * tsp_two
                    + ingredients[2].capacity * tsp_three
                    + ingredients[3].capacity * tsp_four;
                let durability = ingredients[0].durability * tsp_one
                    + ingredients[1].durability * tsp_two
                    + ingredients[2].durability * tsp_three
                    + ingredients[3].durability * tsp_four;
                let flavor = ingredients[0].flavor * tsp_one
                    + ingredients[1].flavor * tsp_two
                    + ingredients[2].flavor * tsp_three
                    + ingredients[3].flavor * tsp_four;
                let texture = ingredients[0].texture * tsp_one
                    + ingredients[1].texture * tsp_two
                    + ingredients[2].texture * tsp_three
                    + ingredients[3].texture * tsp_four;
                if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
                    continue;
                }

                let score = capacity * durability * flavor * texture;
                if score.cmp(&best) == Ordering::Greater {
                    best = score;
                }
            }
        }
    }

    best
}

fn find_score_two(ingredients: &[Ingredient]) -> i32 {
    let mut best = i32::MIN;

    for tsp_one in 0..101 {
        for tsp_two in 0..(101 - tsp_one) {
            for tsp_three in 0..(101 - tsp_one - tsp_two) {
                let tsp_four = 100 - tsp_one - tsp_two - tsp_three;
                let capacity = ingredients[0].capacity * tsp_one
                    + ingredients[1].capacity * tsp_two
                    + ingredients[2].capacity * tsp_three
                    + ingredients[3].capacity * tsp_four;
                let durability = ingredients[0].durability * tsp_one
                    + ingredients[1].durability * tsp_two
                    + ingredients[2].durability * tsp_three
                    + ingredients[3].durability * tsp_four;
                let flavor = ingredients[0].flavor * tsp_one
                    + ingredients[1].flavor * tsp_two
                    + ingredients[2].flavor * tsp_three
                    + ingredients[3].flavor * tsp_four;
                let texture = ingredients[0].texture * tsp_one
                    + ingredients[1].texture * tsp_two
                    + ingredients[2].texture * tsp_three
                    + ingredients[3].texture * tsp_four;
                let calories = ingredients[0].calories * tsp_one
                    + ingredients[1].calories * tsp_two
                    + ingredients[2].calories * tsp_three
                    + ingredients[3].calories * tsp_four;
                if capacity <= 0
                    || durability <= 0
                    || flavor <= 0
                    || texture <= 0
                    || calories != 500
                {
                    continue;
                }

                let score = capacity * durability * flavor * texture;
                if score.cmp(&best) == Ordering::Greater {
                    best = score;
                }
            }
        }
    }

    best
}

pub fn part_one(input: &str) -> i32 {
    find_score_one(&parse(input))
}

pub fn part_two(input: &str) -> i32 {
    find_score_two(&parse(input))
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 15), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use aoc::read_file;
        let input = read_file("examples", 15);
        let ingredients = parse(&input);
        assert_eq!(ingredients[0].capacity, -1);
        assert_eq!(ingredients[0].durability, -2);
        assert_eq!(ingredients[0].flavor, 6);
        assert_eq!(ingredients[0].texture, 3);
        assert_eq!(ingredients[0].calories, 8);
        assert_eq!(ingredients[1].capacity, 2);
        assert_eq!(ingredients[1].durability, 3);
        assert_eq!(ingredients[1].flavor, -2);
        assert_eq!(ingredients[1].texture, -1);
        assert_eq!(ingredients[1].calories, 3);
    }
}
