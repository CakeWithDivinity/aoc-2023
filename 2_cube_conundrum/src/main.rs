use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    fn from_cube_string(id: usize, cubes: &str) -> Self {
        let (mut red, mut green, mut blue): (usize, usize, usize) = (1, 1, 1);
        let draws = cubes.split(';');

        for draw in draws {
            let cube_draws = draw.split(", ");

            for cube_draw in cube_draws {
                let cube_draw = cube_draw.trim_start();

                let (amount, color) = cube_draw.split_once(' ').expect("splits at whitespace");
                let amount: usize = amount.parse().expect("number string");

                match color {
                    "red" => {
                        if amount > red {
                            red = amount;
                        }
                    }
                    "green" => {
                        if amount > green {
                            green = amount;
                        }
                    }
                    "blue" => {
                        if amount > blue {
                            blue = amount;
                        }
                    }
                    _ => panic!("unexpected color string: {color}"),
                }
            }
        }

        Game {
            id,
            red,
            green,
            blue,
        }
    }
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let lines = reader.lines();

    let sum: usize = lines
        .map(|line| {
            let line = line.expect("parses");

            let (game, cubes) = line.split_once(':').expect(": splits game and cubes");
            let (_, game_id) = game.split_at(5);
            let game_id: usize = game_id.parse().expect("parses to number");

            Game::from_cube_string(game_id, cubes)
        })
        .map(|game| game.red * game.blue * game.green)
        .sum();

    println!("sum: {sum}");

    Ok(())
}
