mod input;

fn main() {
    part_two();
}

fn part_one() {
    let mut count = 0;
    for i in 1..input::INPUT.len() {
        if input::INPUT[i - 1] < input::INPUT[i] {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part_two() {
    let mut count = 0;
    for i in 3..input::INPUT.len() {
        if input::INPUT[i - 3] < input::INPUT[i] {
            count += 1;
        }
    }
    println!("{}", count);
}