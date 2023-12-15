use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let result: usize = reader
        .lines()
        .map(|line| {
            let line = line.expect("parseable line");
            line.split(',')
                .map(|line| {
                    line.chars().fold(0, |mut acc, c| {
                        acc += c as usize;
                        acc *= 17;

                        acc % 256
                    })
                })
                .sum::<usize>()
        })
        .sum();

    println!("Result: {result}");

    Ok(())
}
