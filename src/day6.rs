use std::io::{BufReader, BufRead};
use crate::BoxResult;
use std::fs::File;
use std::collections::{HashMap, HashSet};

struct Space {
    orbits: HashMap<String, String>,
    planets: Vec<String>,
}

fn build_space(input: BufReader<File>) -> Space {
    let mut space = Space {
        orbits: HashMap::new(),
        planets: Vec::new(),
    };

    for line in input.lines() {
        let line_val = line.unwrap();
        let vals: Vec<&str> = line_val.split(")").collect();
        let orbitee = vals[0];
        let orbiter = vals[1];
        space.orbits.insert(orbiter.to_string(), orbitee.to_string());
        space.planets.push(orbiter.to_string());
    }
    space
}

pub fn p1(input: BufReader<File>) -> BoxResult<String> {
    let space = build_space(input);

    let mut total = 0;
    for planet in space.planets {
        let mut current_planet = &planet;
        while space.orbits.contains_key(current_planet) {
            total = total + 1;
            current_planet = space.orbits.get(current_planet).unwrap();
        }
    }
    Ok(format!("{}", total))
}

fn all_orbits_for<'a>(space: &'a Space, origin: &'a str) -> HashSet<&'a str> {
    let mut all_orbits = HashSet::new();
    let mut current_planet = origin;

    while space.orbits.contains_key(current_planet) {
        all_orbits.insert(current_planet);
        current_planet = space.orbits.get(current_planet).unwrap();
    }
    all_orbits
}

pub fn p2(input: BufReader<File>) -> BoxResult<String> {
    let space = build_space(input);

    let my_orbits= all_orbits_for(&space, "YOU");
    let santa_orbits = all_orbits_for(&space, "SAN");
    let orbits_in_common = my_orbits.intersection(&santa_orbits).copied().collect();
    let my_unique_orbits = my_orbits.difference(&orbits_in_common);
    let santa_unique_orbits = santa_orbits.difference(&orbits_in_common);

    let num_path = my_unique_orbits.count() + santa_unique_orbits.count() - 2;

    Ok(format!("{}", num_path))

}
