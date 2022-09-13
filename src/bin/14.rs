use std::cmp::Ordering;

struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
    points: u32,
}

impl Reindeer {
    fn distance(&self, seconds: u32) -> u32 {
        let round_time = self.fly_time + self.rest_time;
        let rounds = seconds / round_time;
        let remainder = u32::min(self.fly_time, seconds % round_time);
        (rounds * self.fly_time + remainder) * self.speed
    }
}

fn parse(input: &str) -> Vec<Reindeer> {
    let mut reindeer: Vec<Reindeer> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        reindeer.push(Reindeer {
            speed: parts[3].parse::<u32>().unwrap(),
            fly_time: parts[6].parse::<u32>().unwrap(),
            rest_time: parts[13].parse::<u32>().unwrap(),
            points: 0
        });
    }
    reindeer
}

fn get_winning_distance(reindeer: &[Reindeer], seconds: u32) -> u32 {
    let mut best = u32::MIN;
    for r in reindeer.iter() {
        let distance = r.distance(seconds);
        if distance > best {
            best = distance;
        }
    }
    best
}

fn get_winning_points(reindeer: &mut [Reindeer], seconds: u32) -> u32 {
    for i in 1..(seconds + 1) {
        give_points_to_winning_reindeer(reindeer, i);
    }
    let mut best = u32::MIN;
    for r in reindeer.iter() {
        if r.points > best {
            best = r.points;
        }
    }
    best
}

fn give_points_to_winning_reindeer(reindeer: &mut [Reindeer], seconds: u32) {
    let mut best = u32::MIN;
    let mut best_reindeer: Vec<&mut Reindeer> = vec![];
    for r in reindeer.iter_mut() {
        let distance = r.distance(seconds);
        match distance.cmp(&best) {
            Ordering::Greater => {
                best = distance;
                best_reindeer = vec![r];
            },
            Ordering::Equal => {
                best_reindeer.push(r);
            },
            _ => {},
        }
    }
    for r in best_reindeer.iter_mut() {
        r.points += 1;
    }
}

pub fn part_one(input: &str) -> u32 {
    get_winning_distance(&parse(input), 2503)
}

pub fn part_two(input: &str) -> u32 {
    let mut reindeer = parse(input);
    get_winning_points(&mut reindeer, 2503)
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 14), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        let reindeer = parse(&input);
        assert_eq!(reindeer[0].speed, 14);
        assert_eq!(reindeer[0].fly_time, 10);
        assert_eq!(reindeer[0].rest_time, 127);
        assert_eq!(reindeer[1].speed, 16);
        assert_eq!(reindeer[1].fly_time, 11);
        assert_eq!(reindeer[1].rest_time, 162);
    }

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        let reindeer = parse(&input);
        assert_eq!(get_winning_distance(&reindeer, 1000), 1120);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        let mut reindeer = parse(&input);
        assert_eq!(get_winning_points(&mut reindeer, 1000), 689);
    }
}
