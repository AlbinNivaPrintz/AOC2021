use aoc_utils::read_lines;
use std::collections::{HashMap, HashSet};

fn main() {
    let map = get_input("input");

    let map = make_tiled_input(&map);
    // for row in &map {
    //     for el in row {
    //         print!("{}", el);
    //     }
    //     print!("\n");
    // }

    let res = a_star((0, 0), (map.len() - 1, map[0].len() - 1), &map, taxi);
    println!("{}", res);
}

fn get_input(f: &str) -> Vec<Vec<u32>> {
    let mut map = Vec::new();

    let lines = read_lines(f).unwrap().filter_map(|s| s.ok());

    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap())
        }
        map.push(row);
    }

    map
}

fn make_tiled_input(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut out = Vec::with_capacity(map.len() * 6);
    for row_tiles in 0..5 {
        for row in map.iter() {
            let mut out_row = Vec::with_capacity(map[0].len() * 6);
            for column_tiles in 0..5 {
                for element in row.iter() {
                    let mut new_element = *element + column_tiles + row_tiles;
                    while new_element > 9 {
                        new_element -= 9;
                    }
                    out_row.push(new_element);
                }
            }
            out.push(out_row);
        }
    }
    out
}

fn a_star(
    start: (usize, usize),
    end: (usize, usize),
    map: &Vec<Vec<u32>>,
    h: fn(&(usize, usize), &(usize, usize)) -> u32,
) -> u32 {
    // The set of discovered nodes
    // Only start node
    let mut discovered_nodes = HashSet::new();
    discovered_nodes.insert(start);

    // For node n, stores the node preceding it with the cheapest path to it
    let mut came_from = HashMap::new();

    // for node n, storest the cost of the current cheapest path from start to n
    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    // for node n, the best guess of the cost from going from start to finish if passing trough n
    let mut f_score = HashMap::new();
    f_score.insert(start, h(&start, &end));

    while discovered_nodes.len() > 0 {
        let to_update = *discovered_nodes
            .iter()
            .min_by(|&x, &y| {
                f_score.entry(*x).or_insert(u32::MAX);
                f_score.entry(*y).or_insert(u32::MAX);
                f_score[x].cmp(&f_score[y])
            })
            .unwrap();

        discovered_nodes.remove(&to_update);

        if to_update == end {
            break
        }


        if *g_score.get(&to_update).unwrap() >= *g_score.get(&end).unwrap_or(&u32::MAX)
        {
            continue;
        }

        let neighbours = neighbours_of(&to_update, &end);
        for neighbour in neighbours.iter() {
            let current_g_score = g_score[&to_update] + map[neighbour.0][neighbour.1];

            g_score.entry(*neighbour).or_insert(u32::MAX);
            if current_g_score < g_score[neighbour] {
                // This is better than any previous
                came_from.insert(*neighbour, to_update);
                g_score.insert(*neighbour, current_g_score);
                f_score.insert(*neighbour, current_g_score + h(neighbour, &end));

                discovered_nodes.insert(*neighbour);
            }
        }
    }

    *g_score.get(&end).unwrap_or(&u32::MAX)
}

fn taxi(a: &(usize, usize), b: &(usize, usize)) -> u32 {
    ((b.0 as i64 - a.0 as i64).abs() + (b.1 as i64 - a.1 as i64).abs()) as u32
}

fn neighbours_of(p: &(usize, usize), max: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(4);
    if p.0 > 0 {
        out.push((p.0 - 1, p.1));
    }
    if p.1 > 0 {
        out.push((p.0, p.1 - 1));
    }
    if p.0 < max.0 {
        out.push((p.0 + 1, p.1));
    }
    if p.1 < max.1 {
        out.push((p.0, p.1 + 1));
    }
    out
}
