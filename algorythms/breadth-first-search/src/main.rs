use std::collections::{HashMap, VecDeque};

fn main() {
    let mut graph: HashMap<&str, VecDeque<&str>> = HashMap::new();
    graph.insert("you", VecDeque::from(["alice", "bob", "claire"]));
    graph.insert("bob", VecDeque::from(["anuj", "peggy"]));
    graph.insert("alice", VecDeque::from(["peggy"]));
    graph.insert("claire", VecDeque::from(["thom", "jonny"]));
    graph.insert("anuj", VecDeque::from([]));
    graph.insert("peggy", VecDeque::from([]));
    graph.insert("thom", VecDeque::from([]));
    graph.insert("jonny", VecDeque::from([]));

    println!("{:?}", search("you", &mut graph));
}

fn search(name: &str, graph: &mut HashMap<&str, VecDeque<&str>>) -> Option<String> {
    let mut search_queue: VecDeque<&str> = VecDeque::new();
    let mut searched: Vec<&str> = Vec::new();
    match graph.get_mut(name) {
        Some(names) => search_queue.append(names),
        None => return None,
    }

    while search_queue.len() > 0 {
        let person = search_queue.pop_front().unwrap();
        if searched.contains(&person) {
            continue;
        }
        println!("Cheking: {}", person);
        if person_is_seller(&person) {
            println!("{} is meth seller!", person);
            return Some(person.to_string());
        } else {
            search_queue.append(&mut graph.get_mut(person).unwrap());
            searched.push(person);
        }
    }
    None
}

fn person_is_seller(person: &str) -> bool {
    person.contains("nn")
}
