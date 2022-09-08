use aoc::_13::SeatingInfo;
use itertools::Itertools;
use std::cmp::Ordering;

fn find_happiness(info: &SeatingInfo, ordering: Ordering) -> Result<i32, &'static str> {
    if ordering == Ordering::Equal {
        return Err("Ordering must be either Less or Greater");
    }

    let perms = info
        .seatings
        .iter()
        .permutations(info.seatings.len())
        .unique();
    let mut best = i32::MAX;
    if ordering == Ordering::Greater {
        best = i32::MIN;
    }

    for perm in perms {
        let mut happiness = 0;
        for (i, seating) in perm.iter().enumerate() {
            if i == perm.len() {
                break;
            }
            let other_seating = perm[(i + 1) % perm.len()];

            if *seating == "me" || *other_seating == "me" {
                continue;
            }

            if info.happinesses.get(*seating).is_some()
                && info
                    .happinesses
                    .get(*seating)
                    .unwrap()
                    .get(other_seating)
                    .is_some()
            {
                happiness += info
                    .happinesses
                    .get(*seating)
                    .unwrap()
                    .get(other_seating)
                    .unwrap();
            }
            if info.happinesses.get(other_seating).is_some()
                && info
                    .happinesses
                    .get(other_seating)
                    .unwrap()
                    .get(*seating)
                    .is_some()
            {
                happiness += info
                    .happinesses
                    .get(other_seating)
                    .unwrap()
                    .get(*seating)
                    .unwrap();
            }
        }

        if happiness.cmp(&best) == ordering {
            best = happiness;
        }
    }

    Ok(best)
}

pub fn part_one(input: &str) -> i32 {
    let info = SeatingInfo::new(input);
    find_happiness(&info, Ordering::Greater).unwrap()
}

pub fn part_two(input: &str) -> i32 {
    let mut info = SeatingInfo::new(input);
    info.seatings.insert("me".to_string());
    find_happiness(&info, Ordering::Greater).unwrap()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 13), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_one(&input), 330);
    }
}
