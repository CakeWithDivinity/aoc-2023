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
        let numbers = line
            .split_whitespace()
            .map(|item| item.parse::<isize>().expect("number"))
            .collect::<Vec<isize>>();

        let mut chart = vec![numbers];
        while chart
            .last()
            .expect("always one chart")
            .iter()
            .any(|e| *e != 0)
        {
            let numbers = chart.last().unwrap();

            chart.push(
                numbers
                    .iter()
                    .zip(numbers.iter().skip(1))
                    .map(|(prev, next)| next - prev)
                    .collect(),
            );
        }

        chart.reverse();

        for i in 0..chart.len() {
            if i == 0 {
                chart[i].push(0);
                continue;
            }

            let next = chart[i].last().expect("always one item")
                + chart[i - 1].last().expect("always one item");

            chart[i].push(next);
        }

        sum += chart.last().unwrap().last().unwrap();
    }

    println!("{sum}");

    Ok(())
}
