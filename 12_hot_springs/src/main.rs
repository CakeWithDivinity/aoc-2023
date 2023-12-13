use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug)]
struct SpringRow {
    map: String,
    springs: Vec<usize>,
}

fn parse_input(line: String) -> SpringRow {
    let (map, springs) = line
        .split_once(' ')
        .expect("map and springs are seperated by a space");

    let springs = springs
        .split(',')
        .map(|group| group.parse::<usize>().expect("group is number"))
        .collect::<Vec<usize>>();

    SpringRow {
        map: map.to_string(),
        springs,
    }
}

fn calculate_max_spring_combinations(map: &[u8], springs: &[usize]) -> usize {
    if springs.is_empty() {
        if map.iter().any(|&entry| entry == b'#') {
            return 0;
        }

        return 1;
    }

    let group_len = springs[0];

    let mut sum = 0;

    for i in 0..map.len() {
        if map[i..].len() < group_len {
            // group cannot fit into rest of map
            break;
        }

        if i > 0 && map[i - 1] == b'#' {
            // we skipped a # so the try is invalid
            break;
        }

        if map[i..i + group_len].iter().any(|&entry| entry == b'.') {
            // group does not fit into rest of current map
            continue;
        }

        if map[i..].len() > group_len && map[i + group_len] == b'#' {
            // group would end with # and would be too great
            continue;
        }

        if map[i..].len() > group_len {
            // there are more combinations to try
            sum += calculate_max_spring_combinations(&map[i + group_len + 1..], &springs[1..]);
        } else if springs.len() == 1 {
            // no more combinations to try
            sum += 1;
        }
    }

    sum
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let sum: usize = reader
        .lines()
        .map(|line| parse_input(line.expect("valid line")))
        .map(|spring_row| {
            println!("calculating {:?}", spring_row);
            calculate_max_spring_combinations(spring_row.map.as_bytes(), &spring_row.springs)
        })
        .sum();

    println!("Sum: {sum}");

    Ok(())
}
