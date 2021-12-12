use aoc_utils::read_lines;

fn main() {
    let input = get_input("test_input");

    println!("{:?}", input);
    let next = step(input);
    println!("{:?}", next);
    println!("{:?}", step(next));
}

fn get_input(f: &str) -> Vec<Vec<u32>> {
    let rows = read_lines(f).unwrap();
    let mut out = Vec::new();
    for row in rows {
        let mut out_row = Vec::new();
        if let Ok(line) = row {
            for c in line.chars() {
                out_row.push(c.to_digit(10).unwrap());
            }
        }
        out.push(out_row);
    }
    out
}

fn step(oct: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new_out = Vec::with_capacity(oct.len());
    for i in 0..oct.len() {
        let row = &oct[i];
        let mut new_row = Vec::with_capacity(row.len());
        for j in 0..row.len() {
            new_row.push(row[j] + 1);
        }
        new_out.push(new_row);
    }

    loop {
        let mut n_flashes = 0;
        // Calculate flashes
        for i in 0..new_out.len() {
            for j in 0..new_out[i].len() {
                if new_out[i][j] > 9 {
                    n_flashes += 1;
                    // Spread the flash
                    let mut x_low = 0;
                    let mut x_high = new_out.len() - 1;
                    if i > 0 {
                        x_low = i - 1;
                    }
                    if i < new_out.len() - 2 {
                        x_high = i + 1;
                    }

                    let mut y_low = 0;
                    let mut y_high = new_out[j].len() - 1;
                    if j > 0 {
                        y_low = j - 1;
                    }
                    if j < new_out[i].len() - 2 {
                        y_high = j + 1;
                    }

                    for x in x_low..=x_high {
                        for y in y_low..=y_high {
                            if new_out[x][y] != 0 {
                                new_out[x][y] += 1;
                            }
                        }
                    }

                    // This cell gets zero
                    new_out[i][j] = 0;
                }
            }
        }
        if n_flashes == 0 {
            break;
        }
    }

    new_out
}
