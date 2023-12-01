use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        let mut digits: Vec<u32> = vec![];

        for (i, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    digits.push(c.to_digit(10).expect("digit"));
                }
                _ => {
                    if let Some((_, x)) =
                        numbers.iter().find(|(str, _)| line[i..].starts_with(*str))
                    {
                        digits.push(*x);
                    }
                }
            }
        }

        sum += 10 * digits.first().expect("at least one number")
            + digits.last().expect("at least one number");
    }

    println!("{sum}");

    Ok(())
}
