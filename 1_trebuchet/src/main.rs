use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        let mut digits: Vec<u32> = vec![];

        for c in line.chars() {
            match c {
                '0'..='9' => {
                    digits.push(c.to_digit(10).expect("digit"));
                }
                _ => (),
            }
        }

        sum += 10 * digits.first().expect("at least one number")
            + digits.last().expect("at least one number");
    }

    println!("{sum}");

    Ok(())
}
