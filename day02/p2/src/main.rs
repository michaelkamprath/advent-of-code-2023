mod cubes;

use std::io;

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();

    let mut power_sum: u32 = 0;

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

        let power = consolidate_games.power();

        println!(
            "The power for Game {:?} is: {:?}",
            game_num, power
        );

        power_sum += power;
    }

    println!("The power sum = {:?}", power_sum);

    Ok(())
}
