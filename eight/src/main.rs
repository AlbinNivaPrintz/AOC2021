use aoc_utils::read_lines;

enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    UNKNOWN,
}

// The implied shift
impl From<Letter> for u8 {
    fn from(l: Letter) -> u8 {
        match l {
            Letter::A => 0,
            Letter::B => 1,
            Letter::C => 2,
            Letter::D => 3,
            Letter::E => 4,
            Letter::F => 5,
            Letter::G => 6,
            Letter::UNKNOWN => 0,
        }
    }
}

impl From<char> for Letter {
    fn from(c: char) -> Letter {
        match c {
            'a' => Letter::A,
            'b' => Letter::B,
            'c' => Letter::C,
            'd' => Letter::D,
            'e' => Letter::E,
            'f' => Letter::F,
            'g' => Letter::G,
            _ => Letter::UNKNOWN,
        }
    }
}

struct Pattern {
    pub letters: Vec<Letter>,
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Pattern {
        Pattern {
            letters: s.chars().map(|x| x.into()).collect(),
        }
    }
}

// Turns a pattern into it's binary form.
impl From<Pattern> for u8 {
    fn from(p: Pattern) -> u8 {
        let mut out: u8 = 0;
        for letter in p.letters {
            out |= 1 << u8::from(letter);
        }
        out
    }
}

fn main() -> std::io::Result<()> {
    let lines = read_lines("input").unwrap();

    let mut total_easy = 0;
    let mut total_output_values = 0;
    for line in lines {
        if let Ok(row) = line {
            total_easy += easy_matches_in_row(&row);
            total_output_values += get_output_value(&row);
        }
    }

    println!("{}", total_easy);
    println!("{}", total_output_values);
    Ok(())
}

fn easy_matches_in_row(row: &String) -> u32 {
    let mut sep = row.split(" | ");
    let _ = sep.next().unwrap();
    let output = sep.next().unwrap();

    let easy_patterns: Vec<u8> = output
        .split(" ")
        .filter_map(|pattern| pattern_to_digit(pattern))
        .collect();
    easy_patterns.len() as u32
}

fn get_output_value(row: &String) -> u32 {
    let mut sep = row.split(" | ");
    let signal = sep.next().unwrap();
    let output = sep.next().unwrap();

    let m = get_mapping(signal);

    let mut output_value = 0;
    // mapping is done
    for (i, pattern) in output.split(" ").enumerate() {
        let p: Pattern = pattern.into();
        let target_code = u8::from(p);
        for (mapped_digit, code) in m.iter().enumerate() {
            if target_code == *code {
                output_value += mapped_digit as u32 * 10_u32.pow(3 - i as u32);
                break;
            }
        }
    }

    output_value
}

fn get_mapping(signal: &str) -> [u8; 10] {
    let mut m = [0; 10];

    // Figure out 7, 1, 4, 8
    for pattern in signal.split(" ") {
        if let Some(digit) = pattern_to_digit(pattern) {
            let p: Pattern = pattern.into();
            let code = u8::from(p);
            m[digit as usize] = code;
        }
    }

    let a = m[7] & !m[1];
    let bd = m[4] & !m[1];

    let code_map = signal.split(" ").map(|x| {
        let p: Pattern = x.into();
        u8::from(p)
    });

    for code in code_map.clone() {
        let difference = (code & !(m[4] | a)) | (!code & (m[4] | a));
        if difference.count_ones() == 1 && code != m[4] {
            m[9] = code;
        }
    }

    for code in code_map.clone() {
        let difference = (code & !m[9]) | (!code & m[9]);
        if difference.count_ones() == 1 && code != m[8] {
            if code & m[1] == m[1] {
                m[3] = code;
            } else {
                m[5] = code;
            }
        }
    }

    let e = m[8] & !m[9];
    let c = m[8] & !m[5] & !e;
    let f = m[1] & !c;
    let b = m[5] & !m[3];
    let d = bd & !b;
    m[2] = (m[3] & !f) | e;
    m[0] = m[8] & !d;
    m[6] = (m[9] & !c) | e;
    m
}

fn pattern_to_digit(pattern: &str) -> Option<u8> {
    match pattern.len() {
        3 => Some(7),
        2 => Some(1),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}
