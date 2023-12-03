
use std::fmt::Debug;

pub trait GridEntity: Debug {
	fn location(&self) -> Location;

	fn is_adjacent(&self, other: &(dyn GridEntity)) -> bool;
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Location {
	x: u32,
	y: u32,
}

impl Location {
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			x,
			y,
		}
	}
}


impl GridEntity for Location {
	fn location(&self) -> Location {
		*self
	}

	fn is_adjacent(&self, other: &(dyn GridEntity)) -> bool
	{
		let other_loc = other.location();

		// adjacent if X or Y is +/-1
		let x_start = if other_loc.x == 0 { 0 } else { other_loc.x - 1 };
		let y_start = if other_loc.y == 0 { 0 } else { other_loc.y - 1 };

		(x_start..other_loc.x+2).contains(&self.x) && (y_start..other_loc.y+2).contains(&self.y)
	}
}



#[derive(Debug,Clone)]
pub struct Value {
	locations: Vec<Location>,
	value: u32,
}

impl Value {
	pub fn new(start_location: Location, num_str: &str) -> Self {
		let mut locations: Vec<Location> = Vec::new();

		locations.push(start_location);
		for i in 1..num_str.len() {
			locations.push(Location::new(start_location.x + i as u32, start_location.y));
		}

		let value = match num_str.parse() {
			Ok(v) => v,
			Err(e) => {
				panic!("could not parse string: {:?}, error = {}", num_str, e);
			}
		};

		Self {
			locations,
			value,
		}
	}

	pub  fn value(&self) -> u32 {
		self.value
	}

}

impl GridEntity for Value {
	fn location(&self) -> Location {
		self.locations[0]
	}

	fn is_adjacent(&self, other: &(dyn GridEntity)) -> bool {
		self.locations.iter().any( |&loc| loc.is_adjacent(other))
	}	
}


#[derive(Debug, Clone, Copy)]
pub struct Symbol {
	location: Location,
	symbol: char,
}

impl Symbol {
	pub fn new(location: Location, symbol: char) -> Self {
		Self {
			location,
			symbol,
		}
	}

	pub fn symbol(&self) -> char {
		self.symbol
	}
}

impl GridEntity for Symbol {
	fn location(&self) -> Location {
		self.location
	}

	fn is_adjacent(&self, other: &(dyn GridEntity)) -> bool {
		other.is_adjacent(&self.location)
	}
}

#[cfg(test)]
mod tests {
    use super::{GridEntity, Location, Value, Symbol};

    #[test]
    fn test_location() {
    	let l1 = Location::new(2, 6);
    	let l2 = Location::new(4, 8);
    	let l3 = Location::new(3, 7);
    	let l4 = Location::new(1, 1);
    	let l5 = Location::new(0, 0);

    	assert!(!l1.is_adjacent(&l2));
    	assert!(l2.is_adjacent(&l3));
    	assert!(l3.is_adjacent(&l2));
    	assert!(l1.is_adjacent(&l3));
    	assert!(!l1.is_adjacent(&l4));
    	assert!(l4.is_adjacent(&l5));
    }

    #[test]
    fn test_value() {
    	let v1 = Value::new(Location::new(2,5), "247");

    	assert_eq!(v1.value(), 247);
    	assert!(v1.is_adjacent(&Location::new(5,6)));
    	assert!(!v1.is_adjacent(&Location::new(1,1)));
    }

    #[test]
    fn test_symbol() {
    	let s1 = Symbol::new(Location::new(3,3), '#');

    	assert!(s1.is_adjacent(&Location::new(2,2)));
    }
 }