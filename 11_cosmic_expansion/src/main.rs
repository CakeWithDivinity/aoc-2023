use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let read_space: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut expanded_space = read_space.clone();

    let mut added_row_count = 0;
    for (row_idx, row) in read_space.iter().enumerate() {
        if row.contains(&'#') {
            continue;
        }

        expanded_space.insert(row_idx + added_row_count, row.clone());

        added_row_count += 1;
    }

    let mut added_cols_count = 0;
    for col_idx in 0..expanded_space[0].len() {
        if expanded_space
            .iter()
            .any(|row| row[col_idx + added_cols_count] == '#')
        {
            continue;
        }

        for row in expanded_space.iter_mut() {
            row.insert(col_idx + added_cols_count, row[col_idx + added_cols_count]);
        }

        added_cols_count += 1;
    }

    let mut galaxy_positions: Vec<(usize, usize)> = vec![];
    for (row_idx, row) in expanded_space.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxy_positions.push((row_idx, col_idx));
            }
        }
    }

    let mut galaxy_combinations: Vec<((usize, usize), (usize, usize))> = vec![];
    for i in 0..galaxy_positions.len() {
        for j in i..galaxy_positions.len() {
            galaxy_combinations.push((galaxy_positions[i], galaxy_positions[j]));
        }
    }

    dbg!(galaxy_combinations.len());
    dbg!(galaxy_positions.len());

    let sum: usize = galaxy_combinations
        .iter()
        .map(|(i, j)| {
            (((i.0 as i64) - (j.0 as i64)).abs() + ((i.1 as i64) - (j.1 as i64)).abs()) as usize
        })
        .sum();

    println!("Sum Part 1: {sum}");

    Ok(())
}
