use aoc_utils::read_lines;

#[derive(Debug)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

impl From<&str> for FoldInstruction {
    fn from(s: &str) -> FoldInstruction {
        let mut splat = s.split("=");
        let letter = splat.next().unwrap();
        let number = splat.next().unwrap().parse::<usize>().unwrap();
        match letter {
            "x" => FoldInstruction::X(number),
            "y" => FoldInstruction::Y(number),
            _ => panic!("unknown fold instruction letter: {}", letter),
        }
    }
}

fn main() {
    let (coords, fold_instructions) = get_input("input");
    let mut paper = paper_from_coords(coords);

    for instruction in fold_instructions {
        apply(instruction, &mut paper);
    }
    for row in &paper {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    let mut count = 0;
    for row in &paper {
        for col in row {
            if *col {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn get_input(f: &str) -> (Vec<(usize, usize)>, Vec<FoldInstruction>) {
    let mut coords = Vec::new();
    let lines = read_lines(f).unwrap();

    let line_vec: Vec<String> = lines.filter_map(|x| x.ok()).collect();
    let mut line_iter = line_vec.iter();
    loop {
        let line = line_iter.next().unwrap();
        if line == "" {
            break;
        }
        let mut splat = line.split(",");
        let x = splat.next().unwrap().parse::<usize>().unwrap();
        let y = splat.next().unwrap().parse::<usize>().unwrap();
        coords.push((x, y));
    }

    let mut fold_instructions = Vec::new();
    loop {
        match line_iter.next() {
            Some(line) => {
                // line looks like fold along (x or y)=n
                let instruction = line.split(" ").last().unwrap();
                fold_instructions.push(instruction.into());
            }
            None => break,
        }
    }

    (coords, fold_instructions)
}

fn paper_from_coords(coords: Vec<(usize, usize)>) -> Vec<Vec<bool>> {
    let mut paper = Vec::new();
    paper.push(Vec::new());
    for (x, y) in coords {
        if y >= paper.len() {
            let n_new_rows = y - paper.len() + 1;
            let row_len = paper[0].len();
            for _ in 0..n_new_rows {
                paper.push(std::iter::repeat(false).take(row_len).collect());
            }
        }

        if x >= paper[0].len() {
            let n_new_columns = x - paper[0].len() + 1;
            for row in &mut paper {
                for _ in 0..n_new_columns {
                    row.push(false);
                }
            }
        }

        paper[y][x] = true;
    }
    paper
}

fn apply(instruction: FoldInstruction, paper: &mut Vec<Vec<bool>>) {
    match instruction {
        FoldInstruction::Y(row) => fold_y(paper, row),
        FoldInstruction::X(col) => fold_x(paper, col),
    }
    //
}

fn fold_y(paper: &mut Vec<Vec<bool>>, fold_row: usize) {
    let lim = (paper.len() - fold_row).min(fold_row + 1);
    for i in 1..lim {
        for col in 0..paper[fold_row - i].len() {
            paper[fold_row - i][col] = paper[fold_row - i][col] || paper[fold_row + i][col]
        }
    }
    paper.truncate(fold_row);
}

fn fold_x(paper: &mut Vec<Vec<bool>>, fold_col: usize) {
    let lim = (paper[0].len() - fold_col).min(fold_col + 1);
    for row in 0..paper.len() {
        for i in 1..lim {
            paper[row][fold_col - i] = paper[row][fold_col - i] || paper[row][fold_col + i]
        }
        paper[row].truncate(fold_col);
    }
}
