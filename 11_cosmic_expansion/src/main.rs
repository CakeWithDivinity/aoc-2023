use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(PartialEq, Clone)]
enum Item {
    Empty(usize),
    Galaxy,
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let space: Vec<Vec<Item>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Item::Galaxy,
                    '.' => Item::Empty(1),
                    _ => panic!("unexpected char {c}"),
                })
                .collect()
        })
        .collect();

    let mut expanded_space = space.clone();

    for (row_idx, row) in space.iter().enumerate() {
        if row.contains(&Item::Galaxy) {
            continue;
        }

        expanded_space[row_idx] = vec![Item::Empty(1000000); row.len()];
    }

    for col_idx in 0..expanded_space[0].len() {
        if space.iter().any(|row| row[col_idx] == Item::Galaxy) {
            continue;
        }

        for row in expanded_space.iter_mut() {
            row[col_idx] = Item::Empty(1000000);
        }
    }

    let mut galaxy_positions: Vec<(usize, usize)> = vec![];
    for (row_idx, row) in expanded_space.iter().enumerate() {
        for (col_idx, i) in row.iter().enumerate() {
            if i == &Item::Galaxy {
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

    let sum: usize = galaxy_combinations
        .iter()
        .map(|(i, j)| {
            let mut distance = 0;

            let y_range = if i.0 > j.0 { j.0..i.0 } else { i.0..j.0 };
            for y in y_range {
                distance += match expanded_space[y][i.1] {
                    Item::Galaxy => 1,
                    Item::Empty(x) => x,
                };
            }

            let x_range = if i.1 > j.1 { j.1..i.1 } else { i.1..j.1 };
            for x in x_range {
                distance += match expanded_space[i.0][x] {
                    Item::Galaxy => 1,
                    Item::Empty(x) => x,
                };
            }

            distance
        })
        .sum();

    println!("Sum Part 1: {sum}");

    Ok(())
}
