use std::{
    collections::VecDeque,
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
            .collect::<VecDeque<isize>>();

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

        for i in (0..chart.len()).rev() {
            if i == chart.len() - 1 {
                chart[i].push_front(0);
                continue;
            }

            let next = chart[i].front().expect("always one item")
                - chart[i + 1].front().expect("always one item");

            chart[i].push_front(next);
        }

        sum += chart.first().unwrap().front().unwrap();
    }

    println!("{sum}");

    Ok(())
}
