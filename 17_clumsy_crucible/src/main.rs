use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
    cost: u32,
    direction: Direction,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn is_inverse_of(&self, other: &Direction) -> bool {
        match (self, other) {
            (Direction::North, Direction::South) => true,
            (Direction::South, Direction::North) => true,
            (Direction::West, Direction::East) => true,
            (Direction::East, Direction::West) => true,
            _ => false,
        }
    }

    fn get_idx_diff(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::South,
    Direction::East,
    Direction::North,
    Direction::West,
];

fn find_min_cost_path(map: &[Vec<char>], start_direction: Direction) -> usize {
    let mut queue: BinaryHeap<Point> = BinaryHeap::new();
    let mut checked: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; map[0].len()]; map.len()];
    let mut costs: HashMap<(usize, usize, Direction), u32> = HashMap::new();

    queue.push(Point {
        x: 0,
        y: 0,
        cost: 0,
        direction: start_direction,
    });

    while let Some(point) = queue.pop() {
        if point.y == map.len() - 1 && point.x == map[0].len() - 1 {
            return point.cost as usize;
        }

        if checked[point.y][point.x].contains(&point.direction) {
            continue;
        }

        checked[point.y][point.x].push(point.direction.clone());

        for direction in DIRECTIONS {
            if point.direction.is_inverse_of(&direction) || point.direction == direction {
                continue;
            }

            let (diff_y, diff_x) = direction.get_idx_diff();
            let mut cost_increase = 0;
            for distance in 1..=3 {
                let new_y = point.y.wrapping_add_signed(diff_y * distance);
                let new_x = point.x.wrapping_add_signed(diff_x * distance);

                if new_y >= map.len() || new_x >= map[0].len() {
                    break;
                }

                cost_increase += map[new_y][new_x].to_digit(10).expect("char is digit");

                let new_cost = point.cost + cost_increase;
                let costs_entry = (new_y, new_x, direction.clone());
                let existing_cost = costs.get(&costs_entry).unwrap_or(&u32::MAX);

                if *existing_cost <= new_cost {
                    continue;
                }

                costs.insert(costs_entry, new_cost);
                queue.push(Point {
                    y: new_y,
                    x: new_x,
                    direction: direction.clone(),
                    cost: new_cost,
                })
            }
        }
    }

    unreachable!()
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("parseable line").chars().collect())
        .collect();

    let result1 = std::cmp::min(
        find_min_cost_path(&map, Direction::East),
        find_min_cost_path(&map, Direction::South),
    );

    println!("Part 1: {result1}");

    Ok(())
}
