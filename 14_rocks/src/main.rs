use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn parse_input(reader: BufReader<File>) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

enum TiltDirection {
    North,
    East,
    South,
    West,
}

impl TiltDirection {
    fn get_tile_difference(&self) -> (isize, isize) {
        match self {
            TiltDirection::North => (-1, 0),
            TiltDirection::East => (0, 1),
            TiltDirection::South => (1, 0),
            TiltDirection::West => (0, -1),
        }
    }
}

fn move_rock(row_idx: usize, col_idx: usize, direction: (isize, isize), map: &mut Vec<Vec<char>>) {
    if map[row_idx][col_idx] != 'O' {
        return;
    }

    let (y, x) = direction;

    let (next_y, next_x) = (
        row_idx.wrapping_add_signed(y),
        col_idx.wrapping_add_signed(x),
    );

    if map[next_y][next_x] != '.' {
        return;
    }

    if y != 0 {
        let mut new_row = next_y;

        while new_row > 0
            && new_row < map.len() - 1
            && map[new_row.wrapping_add_signed(y)][col_idx] == '.'
        {
            new_row = new_row.wrapping_add_signed(y);
        }

        map[new_row][col_idx] = 'O';
        map[row_idx][col_idx] = '.';
    }

    if x != 0 {
        let mut new_col = next_x;

        while new_col > 0
            && new_col < map[row_idx].len() - 1
            && map[row_idx][new_col.wrapping_add_signed(x)] == '.'
        {
            new_col = new_col.wrapping_add_signed(x);
        }

        map[row_idx][new_col] = 'O';
        map[row_idx][col_idx] = '.';
    }
}

fn tilt_map(map: &mut Vec<Vec<char>>, direction: &TiltDirection) {
    let (y, x) = direction.get_tile_difference();

    if y != 0 {
        let row_range = if y == 1 {
            (0..map.len() - 1).rev().collect::<Vec<usize>>()
        } else {
            (1..map.len()).collect::<Vec<usize>>()
        };

        for row in row_range {
            for column in 0..map[row].len() {
                move_rock(row, column, (y, x), map);
            }
        }
    }

    if x != 0 {
        let col_range = if x == 1 {
            (0..map[0].len() - 1).rev().collect::<Vec<usize>>()
        } else {
            (1..map[0].len()).collect::<Vec<usize>>()
        };

        for column in col_range {
            for row_idx in 0..map.len() {
                move_rock(row_idx, column, (y, x), map);
            }
        }
    }
}

fn calculate_load(map: &[Vec<char>]) -> usize {
    map.iter()
        .rev()
        .enumerate()
        .map(|(row_idx, row)| {
            let rock_amount = row.iter().filter(|&item| item == &'O').count();

            (row_idx + 1) * rock_amount
        })
        .sum()
}

const DIRECTIONS: [TiltDirection; 4] = [
    TiltDirection::North,
    TiltDirection::West,
    TiltDirection::South,
    TiltDirection::East,
];

fn map_to_string(map: &[Vec<char>]) -> String {
    map.iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut map = parse_input(reader);
    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut iterator = 0..1_000_000_000;
    let mut hit_cache = false;

    while let Some(counter) = iterator.next() {
        DIRECTIONS.iter().for_each(|dir| {
            tilt_map(&mut map, dir);
        });

        let map_string = map_to_string(&map);

        if let Some(cached) = cache.get(&map_string) {
            if !hit_cache {
                println!("Cache hit at {counter}");
                let skippable = counter - cached;
                iterator.nth((((1_000_000_000 - counter) / skippable) * skippable) - 1);
                hit_cache = true;
            }
        }

        cache.insert(map_string, counter);
    }

    let load = calculate_load(&map);
    println!("Load: {load}");

    Ok(())
}
