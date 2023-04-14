use std::collections::HashMap;

type G = HashMap<String, HashMap<String, i32>>;
type C = HashMap<String, i32>;
type P = HashMap<String, Option<String>>;

pub struct Dijkstra {
    pub graph: G,
    pub costs: C,
    pub parents: P,
}

impl Dijkstra {
    pub fn search(&mut self) {
        let mut processed = Vec::new();
        while let Some(node) = self.find_lowest_cost_node(&processed) {
            let cost = self.costs[&node];
            let neighbors = &self.graph[&node];

            neighbors.keys().for_each(|n| {
                let new_cost = cost + neighbors[n];
                if self.costs[n] > new_cost {
                    self.costs.get_mut(n).map(|val| *val = new_cost);
                    self.parents.get_mut(n).map(|val| *val = Some(node.clone()));
                }
            });
            processed.push(node);
        }
    }

    fn find_lowest_cost_node(&self, processed: &Vec<String>) -> Option<String> {
        let mut lowest_cost = i32::MAX;
        let mut lowest_cost_node = None;
        for node in self.costs.keys() {
            let cost = self.costs[node];
            if cost < lowest_cost && !processed.contains(&node.to_string()) {
                lowest_cost = cost;
                lowest_cost_node = Some(node.to_string());
            }
        }
        lowest_cost_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shoud_search_easiest_way() {
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

        dijkstra_expl.search();

        assert_eq!(
            HashMap::from([
                ("a".to_string(), 5),
                ("b".to_string(), 2),
                ("fin".to_string(), 6)
            ]),
            dijkstra_expl.costs
        );
        assert_eq!(
            HashMap::from([
                ("a".to_string(), Some("b".to_string())),
                ("b".to_string(), Some("start".to_string())),
                ("fin".to_string(), Some("a".to_string())),
            ]),
            dijkstra_expl.parents
        );
    }
}
