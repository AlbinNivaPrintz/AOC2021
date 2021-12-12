use aoc_utils::read_lines;

fn close_for_open(c: char) -> Result<char, &'static str> {
    match c {
        '(' => Ok(')'),
        '[' => Ok(']'),
        '{' => Ok('}'),
        '<' => Ok('>'),
        _ => Err("not an opening bracket"),
    }
}

fn corrupted_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn incomplete_score(c: &char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn main() {
    let rows = read_lines("input").unwrap();

    let mut corrupted_score = 0;
    let mut incomplete_scores: Vec<u64> = Vec::new();
    for row in rows {
        if let Ok(line) = row {
            let corrupted = get_corrupted_score(&line);
            corrupted_score += corrupted.0;
            if corrupted.0 == 0 {
                let mut this_incomplete_score = 0;
                for c in corrupted.1.iter().rev() {
                    this_incomplete_score *= 5;
                    this_incomplete_score += incomplete_score(c);
                }
                incomplete_scores.push(this_incomplete_score);
            }
        }
    }
    incomplete_scores.sort();
    println!("corrupted score: {}", corrupted_score);
    println!(
        "incomplete score: {}",
        incomplete_scores[incomplete_scores.len() / 2]
    )
}

fn get_corrupted_score(r: &String) -> (u64, Vec<char>) {
    let mut mem = Vec::new();
    for c in r.chars() {
        if let Ok(b) = close_for_open(c) {
            // Opening new bracket
            mem.push(b);
        } else {
            // Closing a bracket
            if let Some(active) = mem.last() {
                if active == &c {
                    mem.pop();
                } else {
                    return (corrupted_score(c), Vec::new());
                }
            } else {
                return (corrupted_score(c), Vec::new());
            }
        }
    }
    (0, mem)
}
