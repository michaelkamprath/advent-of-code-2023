mod grid;


use std::io;

use grid::{GridEntity, Location, Symbol, Value};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();

    let mut y: u32 = 0;

    let mut values: Vec<Value> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

     while let Some(Ok(line)) = lines.next() {
        let mut i: usize = 0;
        let line_chars: Vec<char> = line.chars().collect();
        while i < line.len() {
            match line_chars[i] {
                '.' => {
                    i += 1;
                },
                '0'..='9' => {
                    // scan ahead to find first non digit or end
                    let num_str: String = line_chars[i..].iter().take_while(|c| c.is_digit(10)).map(|c| *c).collect();

                    values.push(Value::new(Location::new(i as u32, y), num_str.as_str()));

                    i += num_str.len();

                },
                _ => {
                    symbols.push(Symbol::new(Location::new(i as u32, y), line_chars[i]));
                    i += 1;
                },

            }

        }

        y += 1;
     }

     // Part 1
     let mut value_sum = 0;

     for v in &values {
        if symbols.iter().any(|s| v.is_adjacent(s)) {
            value_sum += v.value();
        }
     }

     println!("part 1 value sum = {:?}", value_sum);

     // part 2

    let star_symbols: Vec<Symbol> = symbols.iter().filter(|s| s.symbol() == '*').map(|s| *s).collect();

    let mut gear_ratio_sum: u32 = 0;

    for s in &star_symbols {
        let adjacent_values: Vec<Value> = values.iter().filter(|v| v.is_adjacent(s)).map(|v| v.clone()).collect();

        if adjacent_values.len() == 2 {
            gear_ratio_sum += adjacent_values[0].value()*adjacent_values[1].value();
        }
    }

    println!("part 2 gear ratio sum = {:?}", gear_ratio_sum);
     
     Ok(())
}
