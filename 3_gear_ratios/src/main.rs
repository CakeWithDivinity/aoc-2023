use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn get_adjacent_symbol(
    schematic: &Vec<Vec<char>>,
    line_idx: usize,
    c_idx: usize,
) -> Option<(char, usize, usize)> {
    let mut adj_chars: Vec<(char, usize, usize)> = vec![];

    for &i in &[line_idx.wrapping_sub(1), line_idx, line_idx + 1] {
        for &j in &[c_idx.wrapping_sub(1), c_idx, c_idx + 1] {
            if i < schematic.len() && j < schematic[i].len() {
                if i != line_idx || j != c_idx {
                    let symbol = schematic[i][j];
                    adj_chars.push((symbol, i, j));
                }
            }
        }
    }

    adj_chars.into_iter().find(|(c, _, _)| is_special_symbol(c))
}

fn is_special_symbol(c: &char) -> bool {
    matches!(c, '*' | '#' | '=' | '/' | '&' | '@' | '$' | '+' | '-' | '%')
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let schematic: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| match line {
            Ok(line) => Some(line.chars().collect::<Vec<char>>()),
            Err(_) => None,
        })
        .collect();

    let mut numbers: Vec<usize> = vec![];
    let mut gears: HashMap<String, Vec<usize>> = HashMap::new();

    for (line_idx, line) in schematic.iter().enumerate() {
        let mut c_iter = line.iter().enumerate().peekable();

        while let Some((c_idx, c)) = c_iter.next() {
            match c {
                '0'..='9' => {
                    let mut adj_sym = get_adjacent_symbol(&schematic, line_idx, c_idx);
                    let mut number = c.to_string();

                    while let Some((next_c_idx, next_c)) = c_iter.peek() {
                        if !next_c.is_digit(10) {
                            break;
                        }

                        number.push(**next_c);

                        if adj_sym.is_none() {
                            adj_sym = get_adjacent_symbol(&schematic, line_idx, *next_c_idx);
                        }

                        c_iter.next();
                    }

                    if let Some((c, y, x)) = adj_sym {
                        if c == '*' {
                            let mut gear = gears
                                .get(&format!("x:{}y:{}", x, y))
                                .and_then(|v| Some(v.to_vec()))
                                .unwrap_or(vec![]);

                            gear.push(number.parse().expect("number"));

                            gears.insert(format!("x:{}y:{}", x, y), gear.to_vec());
                        }

                        numbers.push(number.parse().expect("number"));
                    }
                }
                _ => (),
            }
        }
    }

    let gear_ratios = gears
        .iter()
        .filter(|entry| entry.1.len() == 2)
        .map(|(_, nums)| nums.iter().product::<usize>());

    println!("{}", numbers.iter().sum::<usize>());
    println!("{}", gear_ratios.sum::<usize>());

    Ok(())
}
