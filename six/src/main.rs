use aoc_utils::read_lines;

type LanternFish = u8;

fn main() {
    println!("one(80): {}", one(80));
    println!("two(80): {}", two(80));
    println!("two(256): {}", two(256));
}

fn one(n_days: i64) -> usize {
    let mut fishes = get_input("./input");
    for _ in 0..n_days {
        update_fishes(&mut fishes);
    }
    fishes.len()
}

fn get_input(filename: &str) -> Vec<LanternFish> {
    let mut lines = read_lines(filename).unwrap();
    let line = lines.next().unwrap().unwrap();
    let splat = line.split(",");
    splat.map(|x| x.parse().unwrap()).collect()
}

// This is too slow for many days!
fn update_fishes(fishes: &mut Vec<LanternFish>) {
    let mut n_new_fishes = 0;
    for fish in fishes.iter_mut() {
        if *fish == 0 {
            *fish = 6;
            n_new_fishes += 1;
        } else {
            *fish -= 1;
        }
    }
    fishes.extend(std::iter::repeat(8).take(n_new_fishes));
}

fn two(n_days: i64) -> usize {
    let fishes = get_input("./input");

    let mut school = std::collections::VecDeque::from([0; 9]);
    for fish in fishes {
        school[fish as usize] += 1;
    }

    for _ in 0..n_days {
        // Get the number of fishes that had 0 days to offspring
        let n_fishes_producing_offspring = school.pop_front().unwrap();
        school.push_back(n_fishes_producing_offspring);
        school[6] += n_fishes_producing_offspring;
    }

    school.iter().sum()
}
