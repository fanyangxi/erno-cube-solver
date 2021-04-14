use std::fmt;

// red/orange, green/black, yellow/white.
const COLORS: &[char; 7] = &['W', 'R', 'B', 'G', 'Y', 'O', '.'];

pub struct Sticker {
    pub color: char,
}

impl Sticker {
    pub fn new(color: char) -> Self {
        if !COLORS.contains(&color) {
            panic!("Color: ({}) is not one of: {:?}", &color, COLORS);
        }

        Sticker {
            color,
        }
    }

    // __lt__
    // __gt__
    // __le__
    // __ge__
    // __eq__
    // __ne__
}

impl fmt::Display for Sticker {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Sticker: [{}]", self.color)
    }
}

#[cfg(test)]
mod sticker_tests {
    use crate::models::sticker::Sticker;

    #[test]
    fn it_works() {
        let _sticker = Sticker::new('R');
        println!("Cube fmt: [{}].", _sticker);
        // assert_eq!("x", _cube);
    }
}
