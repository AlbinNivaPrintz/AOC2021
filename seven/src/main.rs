use aoc_utils::read_lines;

fn main() {
    let filename = "input";
    let lines = read_lines(filename).unwrap().next();
    let row = lines.unwrap().unwrap();
    let mut numbers: Vec<u32> = row.split(",").map(|x| x.parse().unwrap()).collect();

    let (min, max) = (
        numbers.iter().cloned().min().unwrap(),
        numbers.iter().cloned().max().unwrap(),
    );

    // ONE
    let mut best_fuel_usage = u32::MAX;
    let mut best_alignment = 0;
    for align_to in min..=max {
        let fuel_usage = calculate_fuel(&numbers, align_to, |x| x);
        if best_fuel_usage > fuel_usage {
            best_fuel_usage = fuel_usage;
            best_alignment = align_to;
        }
    }
    println!(
        "one: aligning to {} takes {}",
        best_alignment, best_fuel_usage
    );

    // Another way to do one!
    // I have no idea why this works.
    // My intuition said it would be so, and so it was.
    numbers.sort();
    let median = median(&numbers);
    println!(
        "one v2: aligning to {} takes {}",
        median,
        calculate_fuel(&numbers, median, |x| x)
    );

    // TWO
    let mut best_fuel_usage = u32::MAX;
    let mut best_alignment = 0;
    for align_to in min..=max {
        let fuel_usage = calculate_fuel(&numbers, align_to, |x| x * (x + 1) / 2);
        if best_fuel_usage > fuel_usage {
            best_fuel_usage = fuel_usage;
            best_alignment = align_to;
        }
    }
    println!(
        "two: aligning to {} takes {}",
        best_alignment, best_fuel_usage
    );
}

fn calculate_fuel<F>(ns: &Vec<u32>, align_to: u32, fuel_fn: F) -> u32
where
    F: Fn(u32) -> u32,
{
    let mut fuel_usage = 0;
    for n in ns {
        fuel_usage += fuel_fn(i32::abs(*n as i32 - align_to as i32) as u32);
    }
    fuel_usage
}

// assumes sorted input
fn median(numbers: &Vec<u32>) -> u32 {
    let middle_index = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[middle_index - 1] + numbers[middle_index]) / 2
    } else {
        numbers[middle_index]
    }
}
