use aoc_utils::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
enum CaveType {
    Small,
    Big,
}

impl From<char> for CaveType {
    fn from(c: char) -> CaveType {
        if c.is_lowercase() {
            CaveType::Small
        } else {
            CaveType::Big
        }
    }
}

#[derive(Debug)]
struct Cave {
    pub cave_type: CaveType,
    pub connections: Vec<String>,
    pub visited: bool,
}

impl Cave {
    fn add_connection(&mut self, connection: String) {
        self.connections.push(connection);
    }
}

fn main() {
    let mut g = get_input("input");
    let start = "start".to_owned();
    let mut paths = vec![start.clone()];
    let paths = get_paths(start, &mut g, &mut paths);
    println!("there are {} paths", paths.len());
}

fn get_input(f: &str) -> HashMap<String, Cave> {
    let rows = read_lines(f).unwrap();
    let mut out = HashMap::new();

    for row in rows {
        if let Ok(line) = row {
            let mut splat = line.split("-");
            let from = splat.next().unwrap().to_owned();
            let to = splat.next().unwrap().to_owned();

            let from_type: CaveType = from.chars().next().unwrap().into();
            let to_type: CaveType = to.chars().next().unwrap().into();

            let from_entry = out.entry(from.clone()).or_insert_with(|| Cave {
                cave_type: from_type,
                connections: Vec::new(),
                visited: false,
            });
            from_entry.add_connection(to.clone());

            let to_entry = out.entry(to).or_insert_with(|| Cave {
                cave_type: to_type,
                connections: Vec::new(),
                visited: false,
            });
            to_entry.add_connection(from)
        }
    }
    out
}

fn get_paths(
    start_at: String,
    graph: &mut HashMap<String, Cave>,
    current_path: &mut Vec<String>,
) -> Vec<Vec<String>> {
    let mut generated_paths = Vec::new();
    let mut start = graph.get_mut(&start_at).unwrap();
    start.visited = true;

    let neighbours = graph.get(&start_at).unwrap().connections.clone();
    for neigh_name in neighbours {
        // If this is the end, just add it to the generated paths and continue
        if neigh_name == "end" {
            let mut path_copy = current_path.clone();
            path_copy.push(neigh_name.clone());
            generated_paths.push(path_copy);
            continue;
        }

        let neigh = graph.get_mut(&neigh_name).unwrap();
        if matches!(neigh.cave_type, CaveType::Small) && neigh.visited {
            // This has already been visited!!
            continue;
        }

        // This is not the end, and not an already visited small cave
        // So visit it!
        neigh.visited = true;

        current_path.push(neigh_name.clone());
        let derived_paths = get_paths(neigh_name.clone(), graph, current_path);
        for derived_path in derived_paths {
            generated_paths.push(derived_path);
        }

        {
            // neigh.visited = false;
        }
        current_path.pop();
    }

    let mut start = graph.get_mut(&start_at).unwrap();
    start.visited = false;
    generated_paths
}
