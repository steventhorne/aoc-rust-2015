use std::collections::{HashMap, HashSet};

pub struct SeatingInfo {
    pub seatings: HashSet<Seating>,
    pub happinesses: HashMap<Seating, HashMap<Seating, i32>>,
}

pub type Seating = String;

impl SeatingInfo {
    pub fn new(input: &str) -> Self {
        let mut seatings = HashSet::new();
        let mut happinesses: HashMap<Seating, HashMap<Seating, i32>> = HashMap::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            let from_str = parts[0];
            let happiness_type = parts[2];
            let mut happiness = parts[3].parse::<i32>().unwrap();
            let to_str = parts[10].trim_end_matches('.');

            if happiness_type == "lose" {
                happiness *= -1;
            }

            seatings.insert(from_str.to_string());
            seatings.insert(to_str.to_string());

            let from = from_str.to_string();
            let to = to_str.to_string();

            if let std::collections::hash_map::Entry::Vacant(e) =
                happinesses.entry(from_str.to_string())
            {
                let mut map = HashMap::new();
                map.insert(to, happiness);
                e.insert(map);
            } else {
                happinesses.get_mut(&from).unwrap().insert(to, happiness);
            }
        }

        Self {
            seatings,
            happinesses,
        }
    }
}

