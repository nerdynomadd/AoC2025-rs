use std::{collections::VecDeque, time::Instant};

fn main() {
    let file_content =
        std::fs::read_to_string("assets/fourth.txt").expect("Failed to read input file");

    let mut grid = parse_expression(&file_content);

    let start = Instant::now();
    let accessible = find_accessible_rolls(&grid);
    let dur_find = start.elapsed();

    let start = Instant::now();
    let removed = remove_accessible_rolls(&mut grid);
    let dur_remove = start.elapsed();

    println!("Number of accessible rolls: {}", accessible);
    println!("Number of removable rolls: {}", removed);

    println!(
        "Time find_accessible_rolls:   {:?} ({} ms)",
        dur_find,
        dur_find.as_secs_f64() * 1000.0
    );
    println!(
        "Time remove_accessible_rolls: {:?} ({} ms)",
        dur_remove,
        dur_remove.as_secs_f64() * 1000.0
    );
}

#[derive(Clone)]
pub struct Grid {
    pub rows: Vec<Vec<bool>>,
}

pub fn parse_expression(input: &str) -> Grid {
    let mut rows: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<bool> = Vec::new();
        for c in line.chars() {
            if c == '@' {
                row.push(true);
            } else if c == '.' {
                row.push(false);
            } else {
                panic!("Invalid character in grid data");
            }
        }
        rows.push(row);
    }

    Grid { rows }
}

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn compute_degrees(grid: &Grid) -> Vec<Vec<u8>> {
    let h = grid.rows.len();
    let w = grid.rows[0].len();
    let mut degrees = vec![vec![0u8; w]; h];

    for i in 0..h {
        for j in 0..w {
            if grid.rows[i][j] {
                let mut degree = 0;
                for (di, dj) in DIRS.iter() {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0
                        && ni < h as isize
                        && nj >= 0
                        && nj < w as isize
                        && grid.rows[ni as usize][nj as usize]
                    {
                        degree += 1;
                    }
                }
                degrees[i][j] = degree;
            }
        }
    }

    degrees
}

pub fn find_accessible_rolls(grid: &Grid) -> i32 {
    let degrees = compute_degrees(grid);
    let mut accessible_count = 0;

    for i in 0..grid.rows.len() {
        for j in 0..grid.rows[i].len() {
            if grid.rows[i][j] && degrees[i][j] < 4 {
                accessible_count += 1;
            }
        }
    }

    accessible_count
}

pub fn remove_accessible_rolls(grid: &mut Grid) -> i32 {
    let mut degrees = compute_degrees(grid);

    let mut q = VecDeque::<(usize, usize)>::new();

    for i in 0..grid.rows.len() {
        for j in 0..grid.rows[i].len() {
            if grid.rows[i][j] && degrees[i][j] < 4 {
                q.push_back((i, j));
            }
        }
    }

    let mut removed = 0;

    while let Some((i, j)) = q.pop_front() {
        if !grid.rows[i][j] {
            continue;
        }

        grid.rows[i][j] = false;
        removed += 1;

        for (di, dj) in DIRS.iter() {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni >= 0
                && ni < grid.rows.len() as isize
                && nj >= 0
                && nj < grid.rows[i].len() as isize
            {
                let (niu, nju) = (ni as usize, nj as usize);
                if grid.rows[niu][nju] {
                    degrees[niu][nju] -= 1;
                    if degrees[niu][nju] < 4 {
                        q.push_back((niu, nju));
                    }
                }
            }
        }
    }

    removed
}
