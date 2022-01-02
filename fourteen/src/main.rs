use aoc_utils::read_lines;
use std::collections::HashMap;

fn main() {
    let (mut template, rules, last) = get_input("input");
    
    for _ in 0..40 {
        template = apply_rules(template, &rules);
    }

    let repeats = count_repeats(&template, &last);
    let max = repeats
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, v)| v)
        .unwrap();
    let min = repeats
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, v)| v)
        .unwrap();
    println!("{} = {}-{}", max - min, max, min);
}

fn get_input(f: &str) -> (HashMap<String, u128>, HashMap<String, Vec<String>>, char) {
    let lines = read_lines(f).unwrap();
    let mut line_iter = lines.filter_map(|x| x.ok());
    let start = line_iter.next().unwrap().to_owned();

    let mut template = HashMap::new();
    for i in 0..(start.len() - 1) {
        let substr = &start[i..=i + 1];
        template.entry(substr.to_owned()).and_modify(|e| *e += 1).or_insert(1);
    }

    let last = start.chars().last().unwrap();

    line_iter.next();

    let mut rules = HashMap::new();
    for row in line_iter {
        let mut splat = row.split(" -> ");
        let from = splat.next().unwrap();

        let mut from_chars = from.chars();
        let to_char = splat.next().unwrap();
        let first = from_chars.next().unwrap().to_string();
        let second = from_chars.next().unwrap().to_string();
        rules.insert(
            from.to_owned(),
            vec![first + to_char, to_char.to_owned() + &second],
        );
    }
    (template, rules, last)
}

fn apply_rules(
    template: HashMap<String, u128>,
    rules: &HashMap<String, Vec<String>>,
) -> HashMap<String, u128> {
    let mut out: HashMap<String, u128> = HashMap::with_capacity(template.len());

    for (part, count) in template.iter() {
        if let Some(replacements) = rules.get(part) {
            // If part is in rules, increment replacements
            for replacement in replacements.iter() {
                let new_count = out.entry(replacement.to_string()).or_default();
                *new_count += count;
            }
        } else {
            // else, increment part
            let new_count = out.entry(part.to_string()).or_default();
            *new_count += count;
        }
    }
    out
}

fn count_repeats(s: &HashMap<String, u128>, last: &char) -> HashMap<char, u128> {
    let mut counts = HashMap::new();
    for (part, count) in s.iter() {
        let current = counts.entry(part.chars().next().unwrap()).or_insert(0);
        *current += count;
    }

    counts.entry(*last).and_modify(|e| *e += 1).or_insert(1);

    counts
}
