use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn extract_numbers(line: String) -> usize {
    line.split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .expect("number")
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

    let time = extract_numbers(lines.next().expect("time line").unwrap());
    let distance_record = extract_numbers(lines.next().expect("record line").unwrap());

    let result: usize = calc_win_strategy_amount(time, distance_record);

    println!("Result: {result}");

    Ok(())
}
