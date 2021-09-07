use crate::infra::Problem;
use std::collections::HashMap;

pub struct Day6;

impl Problem<String, String, i32, i32> for Day6 {
    fn day() -> u8 {
        6
    }
    fn first(contents: String) -> i32 {
        let orbits = get_orbits(&contents);
        let mut prev = vec!["COM"];
        let (mut depth, mut res) = (0, 0);
        while !prev.is_empty() {
            res += depth * (prev.len() as i32);
            let mut next = Vec::new();
            for s in prev {
                if let Some(sats) = orbits.get(s) {
                    next.extend(sats);
                }
            }
            prev = next;
            depth += 1;
        }
        res
    }
    fn second(contents: String) -> i32 {
        let orbits = get_orbits(&contents);
        let path_to_you = find_path(&orbits, "COM", "YOU").unwrap();
        let path_to_santa = find_path(&orbits, "COM", "SAN").unwrap();
        let common = path_to_you
            .iter()
            .rev()
            .zip(path_to_santa.iter().rev())
            .position(|(a, b)| a != b)
            .unwrap();
        (path_to_you.len() + path_to_santa.len() - 2 * common - 2) as i32
    }
}

fn get_orbits(contents: &str) -> HashMap<&str, Vec<&str>> {
    let mut orbits = HashMap::<&str, Vec<&str>>::new();
    for (c, s) in contents.lines().map(|o| o.split_at(3)) {
        orbits.entry(c).or_insert_with(Vec::new).push(&s[1..]);
    }
    orbits
}

fn find_path<'a>(
    orbits: &HashMap<&'a str, Vec<&'a str>>,
    from: &'a str,
    to: &'a str,
) -> Option<Vec<&'a str>> {
    if from == to {
        Some(vec![to])
    } else {
        let satellites = orbits.get(from)?;
        let mut path = satellites
            .iter()
            .flat_map(|s| find_path(orbits, s, to))
            .next()?;
        path.push(from);
        Some(path)
    }
}
