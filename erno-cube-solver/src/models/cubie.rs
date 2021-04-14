use crate::models::face::Face;
use std::fmt;
use std::collections::HashMap;
use crate::models::sticker::Sticker;
use std::borrow::Borrow;

// front/back, right/left, up/down.
// const FACINGS: &str = "FBRLUD";
const FACINGS: &[char; 6] = &['F', 'B', 'R', 'L', 'U', 'D'];

// red/orange, green/black, yellow/white.
// const COLORS: &str = "ROGBYW";
const COLORS: &[char; 6] = &['W', 'R', 'B', 'G', 'Y', 'O'];

pub struct Cubie {
    pub facings: HashMap<char, Sticker>,
}

impl Cubie {
    pub fn new(init_key_values: HashMap<char, char>) -> Self {
        let mut results: HashMap<char, Sticker> = HashMap::new();
        for (key, value) in init_key_values {
            if !FACINGS.contains(&key) {
                panic!("Facing: ({}) is not one of: {:?}", &key, FACINGS);
            }
            results.insert(key, Sticker { color: value });
        }
        Cubie {
            facings: results,
        }
    }
}

impl fmt::Display for Cubie {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Cubie: {:?}", self.facings.keys().cloned().collect::<Vec<char>>())
    }
}

#[cfg(test)]
mod cubie_tests {
    use crate::models::cubie::Cubie;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let aaa: HashMap<char, char> = [
            ('F', 'R'),
            ('U', 'G'),
        ].iter().cloned().collect();
        let _cubie = Cubie::new(aaa);

        println!("Cube fmt: [{}].", _cubie);
        // assert_eq!("x", _cube);
    }
}
