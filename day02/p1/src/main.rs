mod cubes;

use std::io;

const MAX_RED: u32 = 12;
const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();

    let mut id_sum: u32 = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() != 2 {
            println!("ERROR: line malformed: {:?}", line);
            continue;
        }
        // get game number, ensure starts with "Game "
        if !parts[0].starts_with("Game ") {
            println!("ERROR: line malformed: {:?}", line);
            continue;
        }

        let game_num: u32 = match parts[0][5..].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("ERROR: Game number malformed: {:?}", line);
                continue;
            }
        };

        let mut consolidate_games = cubes::Cubes::default();

        let draws: Vec<&str> = parts[1].split(';').collect();

        if draws.len() == 0 {
            println!("ERROR: There were no draws in line: {:?}", line);
            continue;
        }

        for draw in draws {
            let cubes = match cubes::Cubes::try_from(draw) {
                Ok(value) => value,
                Err(e) => {
                    println!("ERROR: parsing draw: {:?}, error = {:?}", draw, e);
                    continue;
                }
            };
            consolidate_games = consolidate_games.consolidate(&cubes);
        }

        let possible = consolidate_games.possible(MAX_RED, MAX_BLUE, MAX_GREEN);

        println!(
            "The consolidated results for Game {:?} is: {:?} ==> possible = {:?}",
            game_num, consolidate_games, possible
        );

        if possible {
            id_sum += game_num;
        }
    }

    println!("The possible games ID sume = {:?}", id_sum);

    Ok(())
}
