use aoc::_09::LocationInfo;
use itertools::Itertools;
use std::cmp::Ordering;

fn find_distance(info: &LocationInfo, ordering: Ordering) -> Result<u32, &'static str> {
    if ordering == Ordering::Equal {
        return Err("Ordering must be either Less or Greater");
    }

    let perms = info
        .locations
        .iter()
        .permutations(info.locations.len())
        .unique();
    let mut best = u32::MAX;
    if ordering == Ordering::Greater {
        best = 0;
    }

    for perm in perms {
        let mut distance = 0;
        for (i, location) in perm.iter().enumerate() {
            if i == perm.len() - 1 {
                break;
            }
            let other_location = perm[(i + 1) % perm.len()];
            if info.distances.get(*location).is_some()
                && info
                    .distances
                    .get(*location)
                    .unwrap()
                    .get(other_location)
                    .is_some()
            {
                distance += info
                    .distances
                    .get(*location)
                    .unwrap()
                    .get(other_location)
                    .unwrap();
            } else {
                distance += info
                    .distances
                    .get(other_location)
                    .unwrap()
                    .get(*location)
                    .unwrap();
            }
        }

        if distance.cmp(&best) == ordering {
            best = distance;
        }
    }

    Ok(best)
}

pub fn part_one(input: &str) -> u32 {
    let info = LocationInfo::new(input);
    find_distance(&info, Ordering::Less).unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let info = LocationInfo::new(input);
    find_distance(&info, Ordering::Greater).unwrap()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 9), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_one(&input), 605);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_two(&input), 982);
    }
}
