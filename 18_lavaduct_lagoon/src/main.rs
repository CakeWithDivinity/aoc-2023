use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    str::FromStr,
};

#[derive(Debug)]
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
            Direction::West => (0, -1),
            Direction::South => (1, 0),
        }
    }

    fn from_char(c: &char) -> Self {
        match c {
            '0' => Direction::East,
            '1' => Direction::South,
            '2' => Direction::West,
            '3' => Direction::North,
            _ => panic!("Invalid direction char {c}"),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            "U" => Direction::North,
            _ => panic!("unexpected direction char {s}"),
        })
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    length: isize,
    length_2: isize,
    direction_2: Direction,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<&str>>();

        let direction: Direction = split[0].parse().unwrap();
        let length = split[1].parse::<isize>().unwrap();
        let length_2 = isize::from_str_radix(&split[2][2..=6], 16).unwrap();
        let direction_2 = Direction::from_char(&split[2].chars().nth(7).unwrap());

        Ok(Command {
            direction,
            length,
            length_2,
            direction_2,
        })
    }
}

fn shoelace_formula(vertices: &[(isize, isize)]) -> isize {
    let sum: isize = vertices
        .windows(2)
        .map(|verts| (verts[0].0 + verts[1].0) * (verts[0].1 - verts[1].1))
        .sum();

    sum / 2
}

fn picks_theorem(interior_cnt: isize, circumference: isize) -> isize {
    interior_cnt + (circumference / 2) + 1
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let commands: Vec<Command> = reader
        .lines()
        .map(|line| line.expect("parseable line").parse().unwrap())
        .collect();

    let (mut x, mut y) = (0_isize, 0_isize);
    let mut vertices: Vec<(isize, isize)> = vec![(x, y)];
    let mut circumference: isize = 0;

    for command in commands {
        let (diff_y, diff_x) = command.direction_2.to_idx_diff();
        let next_y = y + diff_y * command.length_2;
        let next_x = x + diff_x * command.length_2;

        circumference += command.length_2;
        vertices.push((next_y, next_x));

        x = next_x;
        y = next_y;
    }

    let shoelace_result = shoelace_formula(&vertices);
    let area = picks_theorem(shoelace_result, circumference);

    println!("{area}");

    Ok(())
}
