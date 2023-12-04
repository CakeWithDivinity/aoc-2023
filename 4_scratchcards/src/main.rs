use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    str::Split,
};

fn parse_string_vec(strings: Split<&str>) -> Vec<usize> {
    strings
        .filter_map(|entry| entry.parse::<usize>().ok())
        .collect()
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let mut card_amount: Vec<usize> = std::iter::repeat(1).take(lines.len()).collect();

    let result: usize = lines
        .iter()
        .enumerate()
        .filter_map(|(line_idx, line)| {
            let (_, line) = line.split_once(": ")?;

            let (winners, drawn) = line.split_once(" | ")?;

            let winners = parse_string_vec(winners.split(" "));
            let drawn = parse_string_vec(drawn.split(" "));

            let win_count = drawn.iter().filter(|draw| winners.contains(draw)).count();

            if win_count == 0 {
                return None;
            }

            for i in 1..=win_count {
                card_amount[line_idx + i] += 1 * card_amount[line_idx];
            }

            Some((2 as usize).pow(win_count as u32 - 1))
        })
        .sum();

    println!("Part 1: {result}");
    println!("Part 2: {}", card_amount.iter().sum::<usize>());
    Ok(())
}
