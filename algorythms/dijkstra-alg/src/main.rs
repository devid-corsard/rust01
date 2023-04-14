use dijkstra_alg::Dijkstra;
use std::collections::HashMap;

fn main() {
    let mut dijkstra_expl = Dijkstra {
        graph: HashMap::from([
            (
                "start".to_string(),
                HashMap::from([("a".to_string(), 6), ("b".to_string(), 2)]),
            ),
            ("a".to_string(), HashMap::from([("fin".to_string(), 1)])),
            (
                "b".to_string(),
                HashMap::from([("fin".to_string(), 5), ("a".to_string(), 3)]),
            ),
            ("fin".to_string(), HashMap::new()),
        ]),
        costs: HashMap::from([
            ("a".to_string(), 6),
            ("b".to_string(), 2),
            ("fin".to_string(), i32::MAX),
        ]),
        parents: HashMap::from([
            ("a".to_string(), Some("start".to_string())),
            ("b".to_string(), Some("start".to_string())),
            ("fin".to_string(), None),
        ]),
    };

    println!("start: {:?}", dijkstra_expl.costs);
    println!("start: {:?}", dijkstra_expl.parents);

    dijkstra_expl.search();

    println!("end: {:?}", dijkstra_expl.costs);
    println!("end: {:?}", dijkstra_expl.parents);
}
