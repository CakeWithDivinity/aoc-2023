use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let turn_sequence = lines.next().unwrap().unwrap();
    let turn_sequence = turn_sequence.chars().cycle();

    // skip empty line
    lines.next().unwrap().unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        let (location, turns) = line.split_once(" = ").unwrap();
        let turns = turns.replace(['(', ')'], "");

        let (left, right) = turns.split_once(", ").unwrap();

        map.insert(location.to_owned(), (left.to_owned(), right.to_owned()));
    }

    let mut location = "AAA";
    let mut steps = 0;

    for turn in turn_sequence {
        if location == "ZZZ" {
            break;
        }

        let (left, right) = map.get(location).unwrap();

        location = match turn {
            'L' => left,
            'R' => right,
            _ => panic!("unexpected turn char {turn}"),
        };
        steps += 1;
    }

    println!("Steps: {steps}");

    Ok(())
}
