use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Red,    // R
    Green,  // G
    Orange, // O
    Blue,   // B
    //
    Yellow, // Y
    White,  // W
}

impl fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Red => write!(fmt, "R"),
            Color::Green => write!(fmt, "G"),
            Color::Orange => write!(fmt, "O"),
            Color::Blue => write!(fmt, "B"),
            Color::Yellow => write!(fmt, "Y"),
            Color::White => write!(fmt, "W"),
        }
    }
}

