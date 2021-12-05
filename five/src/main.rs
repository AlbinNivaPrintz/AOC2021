use aoc_utils::read_lines;

type Point = (usize, usize);
type Line = (Point, Point);

fn point_from_string(point_str: &str) -> Point {
    let mut splat = point_str.split(",");
    let x_str = splat.next().unwrap();
    let y_str = splat.next().unwrap();

    let x = x_str.parse().unwrap();
    let y = y_str.parse().unwrap();
    (x, y)
}

fn line_from_string(line_str: &str) -> Line {
    let mut splat = line_str.split(" -> ");
    let from_str = splat.next().unwrap();
    let to_str = splat.next().unwrap();
    let from = point_from_string(from_str);
    let to = point_from_string(to_str);
    (from, to)
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u64>>,
}

impl Grid {
    fn with_dims(dims: Point) -> Grid {
        let mut g = Vec::with_capacity(dims.1);
        for _ in 0..=dims.1 {
            g.push([0].repeat(dims.0 + 1));
        }
        Grid { grid: g }
    }

    fn draw_horizontal(&mut self, line: &Line) {
        let y = line.0 .1;

        let (beginning, end) = if line.0 .0 < line.1 .0 {
            (line.0 .0, line.1 .0)
        } else {
            (line.1 .0, line.0 .0)
        };

        for x in beginning..=end {
            self.grid[y][x] += 1;
        }
    }

    fn draw_vertical(&mut self, line: &Line) {
        let x = line.0 .0;

        let (beginning, end) = if line.0 .1 < line.1 .1 {
            (line.0 .1, line.1 .1)
        } else {
            (line.1 .1, line.0 .1)
        };

        for y in beginning..=end {
            self.grid[y][x] += 1;
        }
    }

    fn draw_diagonal(&mut self, line: &Line) {
        let (mut x_0, mut x_1) = (line.0 .0, line.1 .0);
        let (mut y_0, mut y_1) = (line.0 .1, line.1 .1);
        if x_0 > x_1 {
            std::mem::swap(&mut x_0, &mut x_1);
            std::mem::swap(&mut y_0, &mut y_1);
        }

        let x_range = x_0..=x_1;
        let y_range: Vec<usize> = if y_0 < y_1 {
            (y_0..=y_1).collect()
        } else {
            (y_1..=y_0).rev().collect()
        };
        for (x, y) in x_range.zip(y_range) {
            self.grid[y][x] += 1;
        }
    }

    fn q1_answer(&self) -> usize {
        let mut count = 0;
        for row in self.grid.iter() {
            for cell in row {
                if *cell >= 2 {
                    count += 1;
                }
            }
        }
        count
    }
}

fn lines_from_file(filename: &str) -> Vec<Line> {
    let mut lines_vec: Vec<Line> = Vec::new();
    let lines = read_lines(filename).unwrap();
    for line in lines {
        if let Ok(row) = line {
            lines_vec.push(line_from_string(&row));
        }
    }
    lines_vec
}

fn main() {
    let lines = lines_from_file("./input");

    let mut dim_x = 0;
    let mut dim_y = 0;
    for line in &lines {
        let (from, to) = line;
        if from.0 > dim_x {
            dim_x = from.0;
        }
        if from.1 > dim_y {
            dim_y = from.1;
        }
        if to.0 > dim_x {
            dim_x = to.0;
        }
        if to.1 > dim_y {
            dim_y = to.1;
        }
    }

    let mut grid = Grid::with_dims((dim_x, dim_y));

    for line in &lines {
        if line.0 .0 == line.1 .0 {
            grid.draw_vertical(line)
        }
        if line.0 .1 == line.1 .1 {
            grid.draw_horizontal(line)
        }
    }

    println!("{}", grid.q1_answer());

    for line in &lines {
        if line.0 .0 != line.1 .0 && line.0 .1 != line.1 .1 {
            grid.draw_diagonal(line)
        }
    }

    println!("{}", grid.q1_answer());
}
