use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn hash_string(input: &str) -> usize {
    input.chars().fold(0, |mut acc, c| {
        acc += c as usize;
        acc *= 17;

        acc % 256
    })
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    for line in reader.lines() {
        let line = line.expect("parseable line");

        for instruction in line.split(',') {
            let instruction_type_idx = instruction
                .chars()
                .position(|c| !c.is_alphanumeric())
                .expect("instruction contains symbol");

            let label = &instruction[0..instruction_type_idx];
            let box_idx = hash_string(label);
            let instruction_type = instruction.as_bytes()[instruction_type_idx];

            match instruction_type {
                b'-' => {
                    if let Some(lens_idx) = boxes[box_idx]
                        .iter()
                        .position(|(item_label, _)| item_label == label)
                    {
                        boxes[box_idx].remove(lens_idx);
                    };
                }
                b'=' => {
                    let focal_length = instruction
                        .chars()
                        .nth(instruction_type_idx + 1)
                        .expect("char after instruction type")
                        .to_digit(10)
                        .expect("last char is digit");

                    match boxes[box_idx]
                        .iter()
                        .position(|(item_label, _)| item_label == label)
                    {
                        Some(existing_idx) => {
                            boxes[box_idx][existing_idx].1 = focal_length as usize;
                        }
                        None => boxes[box_idx].push((label.to_string(), focal_length as usize)),
                    }
                }
                _ => panic!("Unexpected instruction type {instruction_type}"),
            };
        }
    }

    let sum: usize = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, bx)| {
            bx.iter()
                .enumerate()
                .map(|(lens_idx, (_, focal_length))| (1 + box_idx) * (1 + lens_idx) * *focal_length)
                .sum::<usize>()
        })
        .sum();

    println!("Result {sum}");

    Ok(())
}
