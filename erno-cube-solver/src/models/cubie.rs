use crate::models::face::Face;
use std::fmt;
use std::collections::HashMap;
use crate::models::sticker::Sticker;
use std::borrow::Borrow;
use std::collections::hash_map::Keys;
use std::alloc::Global;
use std::ops::Index;

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

    pub fn faces(&self) -> Vec<char> {
        self.facings.keys().copied().collect::<Vec<_>>()
    }

    // colors
    pub fn colors(&self) -> Vec<char> {
        self.facings.values().map(|f| f.color).copied().collect::<Vec<_>>()
    }

    pub fn stickers(&self) -> Vec<Sticker> {
        self.facings.values().copied().collect::<Vec<_>>()
    }

    pub fn facing_to_color(facing: char) -> char {
        let result = match COLORS.get(FACINGS.index(facing)) {
            Some(v) => v,
            None => panic!("Failed to facing_to_color, facing:{}.", facing)
        };
        return *result
    }

    pub fn color_to_facing(color: char) -> char {
        let result = match FACINGS.get(COLORS.index(color)) {
            Some(v) => v,
            None => panic!("Failed to color_to_facing, color:{}.", color)
        };
        return *result
    }

    pub fn color_facing(self, color: char) -> char {
        for (key, sticker) in self.facings {
            if color == sticker.color {
                return key;
            }
        }
        panic!("Failed to color_facing, color:{}.", color)
    }
}

// center: todo...

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
