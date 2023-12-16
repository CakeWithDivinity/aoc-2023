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

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("valid line").chars().collect())
        .collect();

    let mut checked: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; map[0].len()]; map.len()];
    let mut queue: VecDeque<(usize, usize, Direction)> = VecDeque::new();
    queue.push_back((0, 0, Direction::East));

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
                Direction::East => queue.push_back((y - 1, x, Direction::North)),
                Direction::South => queue.push_back((y, x - 1, Direction::West)),
                Direction::West => queue.push_back((y + 1, x, Direction::South)),
            },
            '\\' => match dir {
                Direction::North => queue.push_back((y, x - 1, Direction::West)),
                Direction::East => queue.push_back((y + 1, x, Direction::South)),
                Direction::South => queue.push_back((y, x + 1, Direction::East)),
                Direction::West => queue.push_back((y - 1, x, Direction::North)),
            },
            '-' => match dir {
                Direction::North | Direction::South => {
                    queue.push_back((y, x + 1, Direction::East));
                    queue.push_back((y, x - 1, Direction::West));
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

    let result: usize = checked
        .iter()
        .map(|row| row.iter().filter(|cell| !cell.is_empty()).count())
        .sum();

    println!("Sum: {result}");

    Ok(())
}
