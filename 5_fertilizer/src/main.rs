use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    str::FromStr,
    usize,
};

#[derive(Debug)]
struct Conversion {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

impl FromStr for Conversion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s
            .split_whitespace()
            .map(|part| part.parse::<usize>().expect("number"));

        Ok(Conversion {
            source_start: split.next().expect("source start"),
            destination_start: split.next().expect("destination_start"),
            range: split.next().expect("range"),
        })
    }
}

#[derive(Debug)]
struct ConversionStep {
    _from: String,
    to: String,
    conversions: Vec<Conversion>,
}

impl ConversionStep {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut lines = lines.iter();
        let (from, to) = lines
            .next()
            .expect("line")
            .split_once(' ')
            .expect("XYZ map")
            .0
            .split_once("-to-")
            .expect("XYZ-to-XYZ");

        Self {
            _from: from.to_string(),
            to: to.to_string(),
            conversions: lines.map(|l| l.parse().expect("parseable")).collect(),
        }
    }

    fn convert(&self, input: usize) -> (usize, &str) {
        let conversion = self.conversions.iter().find(|conv| {
            let range = conv.destination_start..conv.destination_start + conv.range;

            range.contains(&input)
        });

        let Some(conversion) = conversion else {
            return (input, &self.to)
        };

        let conversion_diff =
            conversion.destination_start as isize - conversion.source_start as isize;

        let output = usize::try_from(input as isize - conversion_diff).expect("convertable");

        (output, &self.to)
    }
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .expect("seed numbers")
        .split(' ')
        .map(|num| num.parse::<usize>().expect("number"))
        .collect::<Vec<usize>>();

    let seeds = seeds
        .iter()
        .zip(seeds.iter().skip(1))
        .step_by(2)
        .flat_map(|(start, range)| *start..*start + *range);

    let mut conversions: Vec<ConversionStep> = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let mut conv_lines: Vec<String> = vec![line];
        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            conv_lines.push(line);
        }

        conversions.push(ConversionStep::from_lines(conv_lines));
    }

    let min_location = seeds
        .map(|seed| {
            let mut last_result = seed;

            for conversion in conversions.iter() {
                let result = conversion.convert(last_result);
                last_result = result.0;
            }

            last_result
        })
        .min()
        .unwrap();

    println!("{}", min_location);

    Ok(())
}
