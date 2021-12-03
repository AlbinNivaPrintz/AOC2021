use aoc_utils::read_lines;

const WIDTH: usize = 12;

fn main() {
    go();
}

fn go() {
    // Get input
    let mut numbers: Vec<i32> = Vec::new();
    if let Ok(rows) = read_lines("./input") {
        for row in rows {
            if let Ok(line) = row {
                let n = i32::from_str_radix(&line[..], 2).unwrap();
                numbers.push(n);
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for shift_from_front in 0..WIDTH {
        let shift = WIDTH - shift_from_front - 1;
        // Find the most prevelent bit in every column
        let target = get_most_frequent_bit(&numbers, shift) as i32;
        if target == 1 {
            gamma_rate += 1 << shift;
        } else {
            epsilon_rate += 1 << shift;
        }
    }
    println!(
        "part 1: {} * {} = {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    // Find oxygen
    let mut oxy_cands = numbers.to_vec();
    let mut shift = WIDTH - 1;
    while oxy_cands.len() > 1 {
        let target = get_most_frequent_bit(&oxy_cands, shift) as i32;
        oxy_cands.retain(|&x| 1 & (x >> shift) == target);
        shift -= 1;
    }
    let oxy = oxy_cands[0];

    let mut co2_cands = numbers.to_vec();
    let mut shift = WIDTH - 1;
    while co2_cands.len() > 1 {
        let target = get_most_frequent_bit(&co2_cands, shift) as i32;
        co2_cands.retain(|&x| 1 & (x >> shift) == 1 - target);
        shift -= 1;
    }
    let co2 = co2_cands[0];
    println!("part 2: {} * {} = {}", oxy, co2, oxy * co2);
}

fn get_most_frequent_bit(numbers: &Vec<i32>, at_bit: usize) -> bool {
    let mut counter = 0;
    for n in numbers {
        if 1 & (n >> at_bit) == 1 {
            counter += 1;
        } else {
            counter -= 1;
        }
    }
    counter >= 0
}
