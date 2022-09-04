use std::collections::{HashMap, HashSet};

pub struct LocationInfo {
    pub locations: HashSet<Location>,
    pub distances: HashMap<Location, HashMap<Location, u32>>,
}

pub type Location = String;

impl LocationInfo {
    pub fn new(input: &str) -> Self {
        let mut locations = HashSet::new();
        let mut distances: HashMap<Location, HashMap<Location, u32>> = HashMap::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split(" to ").collect();
            let parts_2: Vec<&str> = parts[1].split(" = ").collect();

            let location_str = parts[0];
            let other_location_str = parts_2[0];
            let distance = parts_2[1].parse::<u32>().unwrap();

            locations.insert(location_str.to_string());
            locations.insert(other_location_str.to_string());

            let location = location_str.to_string();
            let other_location = other_location_str.to_string();

            if let std::collections::hash_map::Entry::Vacant(e) = distances.entry(location_str.to_string()) {
                let mut map = HashMap::new();
                map.insert(other_location, distance);
                e.insert(map);
            } else {
                distances.get_mut(&location).unwrap().insert(other_location, distance);
            }
        }

        Self {
            locations,
            distances,
        }
    }
}
