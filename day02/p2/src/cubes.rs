use std::cmp::max;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

impl Cubes {
    pub fn new(red: u32, blue: u32, green: u32) -> Self {
        Self { red, blue, green }
    }

    pub fn consolidate(&self, other: &Cubes) -> Cubes {
        Cubes {
            red: max(self.red, other.red),
            blue: max(self.blue, other.blue),
            green: max(self.green, other.green),
        }
    }

    #[allow(dead_code)]
    pub fn possible(&self, red: u32, blue: u32, green: u32) -> bool {
        (self.red <= red) && (self.blue <= blue) && (self.green <= green)
    }

    pub fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

impl TryFrom<&str> for Cubes {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut found_red: u32 = 0;
        let mut found_blue: u32 = 0;
        let mut found_green: u32 = 0;

        // first split on commas
        let parts = value.split(',');

        for part in parts {
            let components: Vec<&str> = part.trim().split(' ').collect();
            if components.len() != 2 {
                return Err(format!("Component malformed: {:?}", components));
            }
            let count: u32 = match components[0].parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Could not parsecount: {:?}", components)),
            };

            match components[1] {
                "red" => {
                    found_red = count;
                }
                "blue" => {
                    found_blue = count;
                }
                "green" => {
                    found_green = count;
                }
                _ => {
                    return Err("Unknown color type".to_string());
                }
            }
        }

        Ok(Cubes::new(found_red, found_blue, found_green))
    }
}

#[cfg(test)]
mod tests {
    use super::Cubes;

    #[test]
    pub fn test_consolidate() {
        let a1 = Cubes::new(3, 4, 5);
        let a2 = Cubes::new(5, 4, 3);

        assert_eq!(a1.consolidate(&a2), Cubes::new(5, 4, 5));

        let a3 = Cubes::new(2, 1, 7);

        assert_eq!(a1.consolidate(&a3).consolidate(&a2), Cubes::new(5, 4, 7));
    }

    #[test]
    pub fn test_possible() {
        let a1 = Cubes::new(3, 4, 5);

        assert!(a1.possible(5, 5, 5));
        assert!(!a1.possible(1, 5, 5));
        assert!(!a1.possible(5, 1, 5));
        assert!(!a1.possible(5, 5, 1));
    }

    #[test]
    pub fn test_parse_str() {
        assert_eq!(
            Cubes::try_from("3 blue, 4 red").ok().unwrap(),
            Cubes::new(4, 3, 0),
        );

        assert_eq!(
            Cubes::try_from(" 5 blue, 4 red, 13 green").ok().unwrap(),
            Cubes::new(4, 5, 13),
        );
    }
}
