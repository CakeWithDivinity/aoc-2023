use core::num;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn parse_input(reader: BufReader<File>) -> Vec<Vec<Vec<char>>> {
    let mut lines = reader.lines();
    let mut results: Vec<Vec<Vec<char>>> = vec![];

    while let Some(Ok(line)) = lines.next() {
        let mut curr_input: Vec<Vec<char>> = vec![line.chars().collect()];

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            curr_input.push(line.chars().collect());
        }

        results.push(curr_input);
    }

    results
}

enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

fn get_mirror(map: &Vec<Vec<char>>) -> Mirror {
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }

    if let Some(vert) = try_vertical_mirror(&map) {
        return Mirror::Vertical(vert);
    }

    if let Some(horiz) = try_horizontal_mirror(&map) {
        return Mirror::Horizontal(horiz);
    }

    panic!("Found neither vertical nor horizontal mirror");
}

fn try_vertical_mirror(map: &Vec<Vec<char>>) -> Option<usize> {
    let mut rotated_map: Vec<Vec<char>> = vec![];

    for column in 0..map[0].len() {
        let mut new_row: Vec<char> = vec![];
        for row in map {
            new_row.push(row[column]);
        }
        rotated_map.push(new_row);
    }

    for line in &rotated_map {
        println!("{}", line.iter().collect::<String>());
    }

    try_horizontal_mirror(&rotated_map)
}

fn try_horizontal_mirror(map: &Vec<Vec<char>>) -> Option<usize> {
    for mirror_index in 1..map.len() {
        let mirror_range = std::cmp::min(mirror_index, map.len() - mirror_index);

        if (1..=mirror_range).all(|compare_index| {
            let line1 = &map[mirror_index - compare_index];
            let line2 = &map[mirror_index + compare_index - 1];

            dbg!(mirror_index, compare_index);
            dbg!(line1, line2);

            if line1.iter().zip(line2.iter()).all(|(x, y)| x == y) {
                return true;
            }

            false
        }) {
            return Some(mirror_index);
        }
    }

    None
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let inputs = parse_input(reader);

    let result: usize = inputs
        .iter()
        .map(|map| get_mirror(map))
        .map(|result| match result {
            Mirror::Vertical(index) => index,
            Mirror::Horizontal(index) => index * 100,
        })
        .sum();

    println!("Result {result}");

    Ok(())
}
