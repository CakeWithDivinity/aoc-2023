use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

use num::integer::lcm;

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let turn_sequence = lines.next().unwrap().unwrap();

    // skip empty line
    lines.next().unwrap().unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        let (location, turns) = line.split_once(" = ").unwrap();
        let turns = turns.replace(['(', ')'], "");

        let (left, right) = turns.split_once(", ").unwrap();

        map.insert(location.to_owned(), (left.to_owned(), right.to_owned()));
    }

    let step_count = map
        .keys()
        .filter(|location| location.ends_with('A'))
        .map(|location| {
            let mut steps: u64 = 0;
            let mut current_location = location;

            for turn in turn_sequence.chars().cycle() {
                if current_location.ends_with('Z') {
                    break;
                }

                let (left, right) = map.get(current_location).unwrap();

                current_location = match turn {
                    'L' => left,
                    'R' => right,
                    _ => panic!("unexpected turn {turn}"),
                };

                steps += 1;
            }

            steps
        })
        .fold(1, lcm);

    println!("Steps: {step_count}");

    Ok(())
}
