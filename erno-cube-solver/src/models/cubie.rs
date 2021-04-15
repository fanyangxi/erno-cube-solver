use std::fmt;
use std::collections::HashMap;
use crate::models::sticker::Sticker;
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
        let mut items = self.facings.keys().map(|f| *f).collect::<Vec<_>>().to_vec();
        items.sort();
        return items;
    }

    // colors
    pub fn colors(&self) -> Vec<char> {
        let mut items = self.facings.values().map(|f| f.color).collect::<Vec<_>>().to_vec();
        items.sort();
        return items;
    }

    pub fn stickers(&self) -> Vec<Sticker> {
        self.facings.values().map(|f| *f).collect::<Vec<_>>()
    }

    pub fn facing_to_color(facing: char) -> char {
        let result = COLORS.index(FACINGS.iter().position(|&r| r == facing).unwrap());
        return *result
    }

    pub fn color_to_facing(color: char) -> char {
        let result = FACINGS.index(COLORS.iter().position(|&r| r == color).unwrap());
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
    fn should_fmt_cubie_correctly() {
        let aaa: HashMap<char, char> = [
            ('F', 'R'),
            ('U', 'G'),
        ].iter().cloned().collect();
        let _cubie = Cubie::new(aaa);

        let expected = format!("{}", _cubie);
        assert_eq!("Cubie: [\'U\', \'F\']", expected);
    }

    #[test]
    fn should_get_cubie_faces() {
        let aaa: HashMap<char, char> = [
            ('F', 'R'),
            ('U', 'G'),
        ].iter().cloned().collect();
        let _cubie = Cubie::new(aaa);

        assert_eq!(vec!['F', 'U'], _cubie.faces());
    }

    #[test]
    fn should_get_cubie_colors() {
        let aaa: HashMap<char, char> = [
            ('F', 'R'),
            ('U', 'G'),
        ].iter().cloned().collect();
        let _cubie = Cubie::new(aaa);

        assert_eq!(vec!['G', 'R'], _cubie.colors());
    }

    #[test]
    fn should_execute_facing_to_color() {
        let result = Cubie::facing_to_color('U');

        assert_eq!('Y', result);
    }

    #[test]
    fn should_execute_color_to_facing() {
        let result = Cubie::color_to_facing('R');

        assert_eq!('B', result);
    }
}
