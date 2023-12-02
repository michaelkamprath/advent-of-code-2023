use std::io;

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();

    let mut total: u32 = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }
        if let Ok((first_digit, last_digit)) = find_digits(&line) {
            let value: u32 = first_digit * 10 + last_digit;
            println!(
                "first digit = {:?}, last digit = {:?} ==> number = {:?}",
                first_digit, last_digit, value
            );
            total += value;
        } else {
            println!("ERROR: could not find digits in: {:?}", line);
        }
    }

    println!("total = {:?}", total);
    Ok(())
}

fn find_digits(line: &str) -> Result<(u32, u32), ()> {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;

    for c in line.chars() {
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
            }
            last_digit = Some(c);
        }
    }

    if first_digit.is_some() && last_digit.is_some() {
        let first_int = first_digit.unwrap().to_digit(10);
        let last_int = last_digit.unwrap().to_digit(10);

        if first_int.is_some() && last_int.is_some() {
            Ok((first_int.unwrap(), last_int.unwrap()))
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
