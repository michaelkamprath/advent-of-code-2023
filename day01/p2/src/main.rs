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
            total += value;
            println!(
                "{:?}: first digit = {:?}, last digit = {:?} ==> number = {:?} : total = {:?}",
                line, first_digit, last_digit, value, total
            );
        } else {
            println!("ERROR: could not find digits in: {:?}", line);
        }
    }

    println!("total = {:?}", total);
    Ok(())
}

fn find_digits(line: &str) -> Result<(u32, u32), ()> {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    let mut value_str: &str = line;

    while !value_str.is_empty() {
        match is_digit(value_str) {
            Ok((value, remaining_str)) => {
                if first_digit.is_none() {
                    first_digit = Some(value);
                }
                last_digit = Some(value);
                value_str = remaining_str;
            }
            Err(remaining_str) => {
                value_str = remaining_str;
            }
        }
    }

    if first_digit.is_some() && last_digit.is_some() {
        let first_int = first_digit.unwrap();
        let last_int = last_digit.unwrap();

        Ok((first_int, last_int))
    } else {
        Err(())
    }
}

const DIGIT_STRS: &'static [&'static str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Returns Some( digit_value, remaining_str) if substring starts with
/// a digit, or the string minus the first character
fn is_digit<'a>(substring: &'a str) -> Result<(u32, &'a str), &'a str> {
    if substring.is_empty() {
        return Err(substring);
    }
    for i in 1..DIGIT_STRS.len() {
        if substring.starts_with(DIGIT_STRS[i]) {
            return Ok((i as u32, &substring[DIGIT_STRS[i].len() - 1..]));
        }
    }

    let first_char = substring.chars().nth(0).unwrap();
    if first_char.is_digit(10) {
        return Ok((first_char.to_digit(10).unwrap(), &substring[1..]));
    }

    Err(&substring[1..])
}

#[cfg(test)]
mod tests {
    use super::{find_digits, is_digit};

    #[test]
    pub fn test_is_digit() {
        let res1 = is_digit("forty5");
        assert!(res1.is_err());
        assert_eq!(res1.expect_err("res1 should be Err"), "orty5");

        let res2 = is_digit("four5");
        assert!(res2.is_ok());
        assert_eq!(res2.expect("res2 should be Ok"), (4, "r5"));

        let res3 = is_digit("4five");
        assert!(res3.is_ok());
        assert_eq!(res3.expect("res3 should be Ok"), (4, "five"));
    }

    #[test]
    pub fn test_find_digits() {
        let res1 = find_digits("dec one, 2023, is cool");
        assert!(res1.is_ok());
        assert_eq!(res1.expect("res1 should be Ok"), (1, 3));

        let res2 = find_digits("one two three four five");
        assert!(res2.is_ok());
        assert_eq!(res2.expect("res2 should be Ok"), (1, 5));

        let res3 = find_digits("3.14159");
        assert!(res3.is_ok());
        assert_eq!(res3.expect("res3 should be Ok"), (3, 9));

        let res4 = find_digits("there is no digits here");
        assert!(res4.is_err());

        let res5 = find_digits("zero string is not a digit");
        assert!(res5.is_err());

        let res6 = find_digits("sddbkzfczc9");
        assert!(res6.is_ok());
        assert_eq!(res6.expect("res6 should be Ok"), (9, 9));

        // this is the twist in the second part. The the requirements do not
        // explicitly state it, spelled out digits can overlap.
        let res7 = find_digits("oneight");
        assert!(res7.is_ok());
        assert_eq!(res7.expect("res7 should be Ok"), (1, 8));

        let res8 = find_digits("twone");
        assert!(res8.is_ok());
        assert_eq!(res8.expect("res8 should be Ok"), (2, 1));
    }
}
