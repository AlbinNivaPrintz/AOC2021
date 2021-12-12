use aoc_utils::read_lines;

fn main() {
    let mut inp = get_input("input");

    let sink_map = convolute(&inp, min_risk);
    println!(
        "one: {}",
        sink_map
            .iter()
            .map::<u32, fn(&Vec<u32>) -> u32>(|x| x.iter().sum())
            .sum::<u32>()
    );
    let mut basin_sizes = calculate_basin_sizes(&sink_map, &mut inp);
    basin_sizes.sort();
    println!(
        "two: {:?}",
        basin_sizes.into_iter().rev().take(3).product::<usize>()
    );
}

fn get_input(filename: &str) -> Vec<Vec<u32>> {
    let mut map = Vec::new();
    let rows = read_lines(filename).unwrap();
    for row in rows {
        let mut map_row = Vec::new();
        if let Ok(line) = row {
            for c in line.chars() {
                let d = c.to_digit(10).unwrap();
                map_row.push(d);
            }
        }
        map.push(map_row);
    }
    map
}

fn min_risk(center: &u32, neighbours: &[u32]) -> u32 {
    for neighbour in neighbours {
        if neighbour <= center {
            return 0;
        }
    }
    center + 1
}

fn convolute(map: &Vec<Vec<u32>>, f: fn(&u32, &[u32]) -> u32) -> Vec<Vec<u32>> {
    let mut out = Vec::with_capacity(map.len());
    for i in 0..map.len() {
        let mut out_row = Vec::with_capacity(map[i].len());
        for j in 0..map[i].len() {
            let center = &map[i][j];
            let neighbours: Vec<u32> = get_neighbours_of(&map, i, j)
                .iter()
                .map(|x| map[x.0][x.1])
                .collect();
            out_row.push(f(center, &neighbours));
        }
        out.push(out_row);
    }
    out
}

fn get_neighbours_of(map: &Vec<Vec<u32>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    if i > 0 {
        neighbours.push((i - 1, j));
    }
    if i < map.len() - 1 {
        neighbours.push((i + 1, j));
    }
    if j > 0 {
        neighbours.push((i, j - 1));
    }
    if j < map[i].len() - 1 {
        neighbours.push((i, j + 1));
    }
    neighbours
}

fn calculate_basin_sizes(sink_map: &Vec<Vec<u32>>, map: &mut Vec<Vec<u32>>) -> Vec<usize> {
    let mut basin_sizes = Vec::new();
    for i in 0..sink_map.len() {
        for j in 0..sink_map[i].len() {
            if sink_map[i][j] > 0 {
                basin_sizes.push(calculate_basin_size(sink_map, map, i, j));
            }
        }
    }
    basin_sizes
}

fn calculate_basin_size(
    sink_map: &Vec<Vec<u32>>,
    map: &mut Vec<Vec<u32>>,
    i: usize,
    j: usize,
) -> usize {
    let mut size = 1;
    for neighbour in get_neighbours_of(map, i, j) {
        if map[i][j] < map[neighbour.0][neighbour.1] && map[neighbour.0][neighbour.1] != 9 {
            size += calculate_basin_size(sink_map, map, neighbour.0, neighbour.1)
        }
    }
    map[i][j] = 9;
    size
}
