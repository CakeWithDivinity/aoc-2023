use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn parse_input(reader: BufReader<File>) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn tilt_north(map: &mut Vec<Vec<char>>) {
    for row in 1..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] != 'O' {
                continue;
            }

            if map[row - 1][column] != '.' {
                continue;
            }

            let mut new_row = row - 1;

            while new_row > 0 && map[new_row - 1][column] == '.' {
                new_row -= 1;
            }

            map[new_row][column] = 'O';
            map[row][column] = '.';
        }
    }
}

fn calculate_load(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .rev()
        .enumerate()
        .map(|(row_idx, row)| {
            let rock_amount = row.iter().filter(|&item| item == &'O').count();

            (row_idx + 1) * rock_amount
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut map = parse_input(reader);
    tilt_north(&mut map);

    let load = calculate_load(&map);
    println!("Load: {load}");

    Ok(())
}
