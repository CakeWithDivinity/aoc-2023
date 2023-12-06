use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn extract_numbers(line: String) -> Vec<usize> {
    line.split_whitespace()
        .skip(1)
        .map(|time| time.parse::<usize>().expect("number"))
        .collect()
}

fn calc_win_strategy_amount(time: usize, distance_record: usize) -> usize {
    let mut count = 0;

    for hold_time in 1..time {
        let speed = hold_time;
        let remaining_time = time - hold_time;
        let distance = speed * remaining_time;

        if distance > distance_record {
            count += 1;
        }
    }

    count
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let times = extract_numbers(lines.next().expect("time line").unwrap());
    let distance_records = extract_numbers(lines.next().expect("record line").unwrap());

    let result: usize = times
        .iter()
        .zip(distance_records)
        .map(|(time, distance_record)| calc_win_strategy_amount(*time, distance_record))
        .product();

    println!("Result: {result}");

    Ok(())
}
