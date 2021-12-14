use aoc_utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Cave {
    pub connections: Vec<String>,
}

impl Cave {
    fn new() -> Cave {
        Cave {
            connections: Vec::new(),
        }
    }
    fn add_connection(&mut self, connection: String) {
        self.connections.push(connection);
    }
}

fn main() {
    let mut g = get_input("input");
    let paths = get_paths(&mut g);
    println!("there are {} paths", paths.len());
    // let mut str_paths: Vec<String> = paths.iter().map(|x| x.join(",")).collect();
    // str_paths.sort();
    // for p in str_paths {
    //     println!("{}", p);
    // }
}

fn get_input(f: &str) -> HashMap<String, Cave> {
    let rows = read_lines(f).unwrap();
    let mut out = HashMap::new();

    for row in rows {
        if let Ok(line) = row {
            let mut splat = line.split("-");
            let from = splat.next().unwrap().to_owned();
            let to = splat.next().unwrap().to_owned();

            let from_entry = out.entry(from.clone()).or_insert_with(|| Cave::new());
            from_entry.add_connection(to.clone());

            let to_entry = out.entry(to.clone()).or_insert_with(|| Cave::new());
            to_entry.add_connection(from)
        }
    }
    out
}

fn get_paths(graph: &mut HashMap<String, Cave>) -> Vec<Vec<String>> {
    let mut paths = vec![vec!["start".to_owned()]];

    loop {
        let mut all_ended = true;
        let mut new_paths = Vec::new();

        for path in paths {
            let leaf_key = path.last().unwrap();

            if leaf_key == "end" {
                new_paths.push(path.clone());
                continue;
            }

            let leaf = graph.get(leaf_key).unwrap();

            for neigh in &leaf.connections {
                if !can_visit(neigh, &path) {
                    continue;
                }

                all_ended = false;

                let mut tmp = path.clone();
                tmp.push(neigh.clone());
                new_paths.push(tmp);
            }
        }

        paths = new_paths;
        if all_ended {
            break;
        }
    }

    paths
}

fn can_visit(s: &String, path: &Vec<String>) -> bool {
    // Check if can add leaf
    // This is stupid because we count the same thing multiple times
    if s == "start" {
        return false;
    }

    let mut used_twice = false;
    let mut hs = HashSet::new();
    for el in path {
        let not_had_already = hs.insert(el);
        if !not_had_already && el.chars().last().unwrap().is_lowercase() {
            used_twice = true
        }
    }

    if s.chars().last().unwrap().is_lowercase() {
        !used_twice || !hs.get(s).is_some()
    } else {
        true
    }
}
