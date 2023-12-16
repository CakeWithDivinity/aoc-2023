use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_idx_diff(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

fn calculate_energized_tiles(
    map: &Vec<Vec<char>>,
    start_row_idx: usize,
    start_col_idx: usize,
    start_dir: Direction,
) -> usize {
    let mut checked: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; map[0].len()]; map.len()];
    let mut queue: VecDeque<(usize, usize, Direction)> = VecDeque::new();
    queue.push_back((start_row_idx, start_col_idx, start_dir));

    while let Some((y, x, dir)) = queue.pop_front() {
        if !(0..map.len()).contains(&y) || !(0..map[0].len()).contains(&x) {
            continue;
        }

        if checked[y][x].contains(&dir) {
            continue;
        }

        let (dir_y, dir_x) = dir.to_idx_diff();
        checked[y][x].push(dir.clone());

        match map[y][x] {
            '.' => {
                queue.push_back((
                    y.wrapping_add_signed(dir_y),
                    x.wrapping_add_signed(dir_x),
                    dir,
                ));
            }
            '/' => match dir {
                Direction::North => queue.push_back((y, x + 1, Direction::East)),
                Direction::East => {
                    queue.push_back((y.wrapping_add_signed(-1), x, Direction::North))
                }
                Direction::South => {
                    queue.push_back((y, x.wrapping_add_signed(-1), Direction::West))
                }
                Direction::West => queue.push_back((y + 1, x, Direction::South)),
            },
            '\\' => match dir {
                Direction::North => {
                    queue.push_back((y, x.wrapping_add_signed(-1), Direction::West))
                }
                Direction::East => queue.push_back((y + 1, x, Direction::South)),
                Direction::South => queue.push_back((y, x + 1, Direction::East)),
                Direction::West => {
                    queue.push_back((y.wrapping_add_signed(-1), x, Direction::North))
                }
            },
            '-' => match dir {
                Direction::North | Direction::South => {
                    queue.push_back((y, x + 1, Direction::East));
                    queue.push_back((y, x.wrapping_add_signed(-1), Direction::West));
                }
                _ => queue.push_back((
                    y.wrapping_add_signed(dir_y),
                    x.wrapping_add_signed(dir_x),
                    dir,
                )),
            },
            '|' => match dir {
                Direction::East | Direction::West => {
                    queue.push_back((y.wrapping_add_signed(-1), x, Direction::North));
                    queue.push_back((y.wrapping_add_signed(1), x, Direction::South));
                }
                _ => queue.push_back((
                    y.wrapping_add_signed(dir_y),
                    x.wrapping_add_signed(dir_x),
                    dir,
                )),
            },
            c => panic!("unexpected char {c}"),
        }
    }

    checked
        .iter()
        .map(|row| row.iter().filter(|cell| !cell.is_empty()).count())
        .sum()
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("valid line").chars().collect())
        .collect();

    println!(
        "Part 1 Sum: {}",
        calculate_energized_tiles(&map, 0, 0, Direction::East)
    );

    let mut max_part_2 = 0;

    for i in 0..map.len() {
        let energized_tiles = calculate_energized_tiles(&map, i, 0, Direction::East);
        if energized_tiles > max_part_2 {
            max_part_2 = energized_tiles;
        }

        let energized_tiles = calculate_energized_tiles(&map, i, map[0].len() - 1, Direction::West);
        if energized_tiles > max_part_2 {
            max_part_2 = energized_tiles;
        }
    }

    for i in 0..map[0].len() {
        let energized_tiles = calculate_energized_tiles(&map, 0, i, Direction::South);
        if energized_tiles > max_part_2 {
            max_part_2 = energized_tiles;
        }

        let energized_tiles = calculate_energized_tiles(&map, map.len() - 1, i, Direction::North);
        if energized_tiles > max_part_2 {
            max_part_2 = energized_tiles;
        }
    }

    println!("Part 2 max: {max_part_2}");

    Ok(())
}
