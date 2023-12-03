use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn has_adjacent_symbol(schematic: &Vec<Vec<char>>, line_idx: usize, c_idx: usize) -> bool {
    let mut adj_chars: Vec<char> = vec![];

    let is_top = line_idx == 0;
    let is_left = c_idx == 0;
    let is_bottom = line_idx == schematic.len() - 1;
    let is_right = c_idx == schematic[line_idx].len() - 1;

    if !is_top {
        if !is_left {
            adj_chars.push(schematic[line_idx - 1][c_idx - 1]);
        }

        adj_chars.push(schematic[line_idx - 1][c_idx]);

        if !is_right {
            adj_chars.push(schematic[line_idx - 1][c_idx + 1]);
        }
    }

    if !is_left {
        adj_chars.push(schematic[line_idx][c_idx - 1]);
    }

    if !is_right {
        adj_chars.push(schematic[line_idx][c_idx + 1]);
    }

    if !is_bottom {
        if !is_left {
            adj_chars.push(schematic[line_idx + 1][c_idx - 1]);
        }

        adj_chars.push(schematic[line_idx + 1][c_idx]);

        if !is_right {
            adj_chars.push(schematic[line_idx + 1][c_idx + 1]);
        }
    }

    adj_chars.iter().any(is_special_symbol)
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

    for (line_idx, line) in schematic.iter().enumerate() {
        let mut c_iter = line.iter().enumerate().peekable();

        while let Some((c_idx, c)) = c_iter.next() {
            match c {
                '0'..='9' => {
                    let mut has_adj_symbol = has_adjacent_symbol(&schematic, line_idx, c_idx);
                    let mut number = c.to_string();

                    while let Some((next_c_idx, next_c)) = c_iter.peek() {
                        if !next_c.is_digit(10) {
                            break;
                        }

                        number.push(**next_c);

                        if !has_adj_symbol {
                            has_adj_symbol = has_adjacent_symbol(&schematic, line_idx, *next_c_idx);
                        }

                        c_iter.next();
                    }

                    if has_adj_symbol {
                        numbers.push(number.parse().expect("number"));
                    }
                }
                _ => (),
            }
        }
    }

    println!("{}", numbers.iter().sum::<usize>());

    Ok(())
}
