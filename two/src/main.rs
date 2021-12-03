use aoc_utils::read_lines;

const F: &str = "forward";
const D: &str = "down";
const U: &str = "up";

fn main() {
    one();
    two();
}

fn two() {
    let mut fw = 0;
    let mut depth = 0;
    let mut aim = 0;
    if let Ok(rows) = read_lines("./input.txt") {
        for row in rows {
            if let Ok(line) = row {
                let mut split = line.split(" ");
                let direction = split.next().unwrap();
                let amount = split.next().unwrap().parse::<u32>().unwrap();
                match direction {
                    F => {
                        fw += amount;
                        depth += aim * amount;
                    }
                    D => aim += amount,
                    U => aim -= amount,
                    _ => panic!("{}", amount),
                }
            }
        }
    }
    println!("{}", fw * depth);
}

fn one() {
    let mut fw = 0;
    let mut depth = 0;
    if let Ok(rows) = read_lines("./input.txt") {
        for row in rows {
            if let Ok(line) = row {
                let mut split = line.split(" ");
                let direction = split.next().unwrap();
                let amount = split.next().unwrap().parse::<u32>().unwrap();
                match direction {
                    F => fw += amount,
                    D => depth += amount,
                    U => depth -= amount,
                    _ => panic!("{}", amount),
                }
            }
        }
    }
    println!("{}", fw * depth);
}
