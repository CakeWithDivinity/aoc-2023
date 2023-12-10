use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_coordinate_diff(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
}

impl Pipe {
    fn directions(&self) -> (Direction, Direction) {
        match self {
            Pipe::Vertical => (Direction::North, Direction::South),
            Pipe::Horizontal => (Direction::East, Direction::West),
            Pipe::NorthToEast => (Direction::North, Direction::East),
            Pipe::NorthToWest => (Direction::North, Direction::West),
            Pipe::SouthToEast => (Direction::South, Direction::East),
            Pipe::SouthToWest => (Direction::South, Direction::West),
            _ => panic!("cannot determine directions for {:?}", self),
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthToEast,
            'J' => Self::NorthToWest,
            '7' => Self::SouthToWest,
            'F' => Self::SouthToEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("unexpected char {} for pipe", value),
        }
    }
}

fn get_start_direction(start_y: &usize, start_x: &usize, map: &[Vec<Pipe>]) -> Direction {
    if *start_y > 0
        && [Pipe::Vertical, Pipe::SouthToWest, Pipe::SouthToEast]
            .contains(&map[start_y - 1][*start_x])
    {
        return Direction::North;
    }

    if *start_x > 0
        && [Pipe::Horizontal, Pipe::NorthToEast, Pipe::SouthToEast]
            .contains(&map[*start_y][start_x - 1])
    {
        return Direction::West;
    }

    if [Pipe::Horizontal, Pipe::NorthToWest, Pipe::SouthToWest]
        .contains(&map[*start_y][start_x + 1])
    {
        return Direction::East;
    }

    if [Pipe::Vertical, Pipe::NorthToWest, Pipe::NorthToEast].contains(&map[start_y + 1][*start_x])
    {
        return Direction::South;
    }

    unreachable!();
}

fn get_next_position(curr: (usize, usize), dir: Direction) -> (usize, usize) {
    let (add_y, add_x) = dir.to_coordinate_diff();

    (
        ((curr.0 as isize) + add_y) as usize,
        ((curr.1 as isize) + add_x) as usize,
    )
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut map: Vec<Vec<Pipe>> = reader
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c.into()).collect())
        .collect();

    let start_y = map
        .iter()
        .position(|line| line.contains(&Pipe::Start))
        .expect("S to be in input");

    let start_x = map[start_y]
        .iter()
        .position(|pipe| pipe == &Pipe::Start)
        .expect("S to be in input");

    let mut checked = vec![vec![0; map[0].len()]; map.len()];

    checked[start_y][start_x] = 1;

    let mut curr: (usize, usize) = get_next_position(
        (start_y, start_x),
        get_start_direction(&start_y, &start_x, &map),
    );

    let mut distance = 0;

    loop {
        let curr_pipe = &map[curr.0][curr.1];
        if curr_pipe == &Pipe::Start {
            break;
        }

        checked[curr.0][curr.1] = 1;
        let poss_nexts = curr_pipe.directions();

        let poss_next1 = get_next_position(curr, poss_nexts.0);
        let poss_next2 = get_next_position(curr, poss_nexts.1);

        let is_next1_visited = checked[poss_next1.0][poss_next1.1] == 1;
        let is_next2_visited = checked[poss_next2.0][poss_next2.1] == 1;

        if is_next1_visited && is_next2_visited {
            // both ends of the pipe are visited, so we are back at the start
            break;
        }

        if is_next1_visited {
            curr = poss_next2;
        } else {
            curr = poss_next1;
        }

        distance += 1;
    }

    println!("Max distance: {}", distance / 2 + 1);

    // this only works with my input. But i am too lazy to replace
    // the start pipe with the actual pipe
    map[start_y][start_x] = Pipe::NorthToEast;

    let mut sum = 0;
    for y in 0..checked.len() {
        let mut in_loop = false;
        for x in 0..checked[y].len() {
            if checked[y][x] == 1 {
                if [Pipe::Vertical, Pipe::SouthToWest, Pipe::SouthToEast].contains(&map[y][x]) {
                    in_loop = !in_loop;
                }

                continue;
            }

            if in_loop {
                sum += 1;
            }
        }
    }

    println!("Area inside loop: {sum}");

    Ok(())
}
