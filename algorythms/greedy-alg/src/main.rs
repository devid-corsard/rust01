use std::collections::{HashMap, HashSet};

fn main() {
    let mut states_needed = HashSet::from_iter(
        ["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]
            .into_iter()
            .map(|i| i.to_string()),
    );
    let stations: HashMap<String, HashSet<String>> = HashMap::from([
        (
            "kone".to_string(),
            HashSet::from_iter(["id", "nv", "ut"].into_iter().map(|i| i.to_string())),
        ),
        (
            "ktwo".to_string(),
            HashSet::from_iter(["id", "wa", "mt"].into_iter().map(|i| i.to_string())),
        ),
        (
            "kthree".to_string(),
            HashSet::from_iter(["or", "nv", "ca"].into_iter().map(|i| i.to_string())),
        ),
        (
            "kfour".to_string(),
            HashSet::from_iter(["nv", "ut"].into_iter().map(|i| i.to_string())),
        ),
        (
            "kfive".to_string(),
            HashSet::from_iter(["ca", "az"].into_iter().map(|i| i.to_string())),
        ),
    ]);
    let mut final_stations: HashSet<&String> = HashSet::new();

    while !states_needed.is_empty() {
        let mut best_station: Option<&String> = None;
        let mut states_covered: HashSet<String> = HashSet::new();

        stations.iter().for_each(|(station, states_for_station)| {
            let covered: HashSet<String> = states_needed
                .intersection(&states_for_station)
                .map(|s: &String| s.to_string())
                .collect();
            if covered.len() > states_covered.len() {
                best_station = Some(station);
                states_covered = covered;
            }
        });

        states_needed = states_needed
            .difference(&states_covered)
            .map(|i| i.to_string())
            .collect();

        final_stations.insert(best_station.expect("best station is None!"));
    }
    println!("{:?}", final_stations);
}
