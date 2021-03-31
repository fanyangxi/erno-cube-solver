use crate::models::face::Face;
use std::fmt;

pub struct Cubie {
    pub front: Face, // Front
    pub back: Face, // Back
    pub left: Face, // Left
    pub right: Face, // Right
    pub up: Face, // Up
    pub down: Face, // Down
}

impl Cubie {
    pub fn new(faces: [Face; 6]) -> Self {
        Cubie {
            front: faces[0],
            back: faces[1],
            left: faces[2],
            right: faces[3],
            up: faces[4],
            down: faces[5],
        }
    }
}

impl fmt::Display for Cubie {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Cube: {} {} {} {} {} {}",
               self.front, self.back,
               self.left, self.right,
               self.up, self.down)
    }
}

#[cfg(test)]
mod cross_tests {
    use crate::models::cube::Cube;
    use crate::models::colors::Color;
    use crate::models::face::Face;

    #[test]
    fn it_works() {

        let face1 = Face::new([
            [Color::Red,Color::Red,Color::Red],
            [Color::Red,Color::Red,Color::Red],
            [Color::Red,Color::Red,Color::Red],
        ]);
        let face2 = Face::new([
            [Color::Green,Color::Green,Color::Green],
            [Color::Green,Color::Green,Color::Green],
            [Color::Green,Color::Green,Color::Green],
        ]);
        let face3 = Face::new([
            [Color::Yellow,Color::Yellow,Color::Yellow],
            [Color::Yellow,Color::Yellow,Color::Yellow],
            [Color::Yellow,Color::Yellow,Color::Yellow],
        ]);
        let face4 = Face::new([
            [Color::Blue,Color::Blue,Color::Blue],
            [Color::Blue,Color::Blue,Color::Blue],
            [Color::Blue,Color::Blue,Color::Blue],
        ]);
        let face5 = Face::new([
            [Color::Orange,Color::Orange,Color::Orange],
            [Color::Orange,Color::Orange,Color::Orange],
            [Color::Orange,Color::Orange,Color::Orange],
        ]);
        let face6 = Face::new([
            [Color::White,Color::White,Color::White],
            [Color::White,Color::White,Color::White],
            [Color::White,Color::White,Color::White],
        ]);

        let _cube = Cube::new([face1, face2, face3, face4, face5, face6]);
        println!("Cube fmt: [{}].", _cube);
        // assert_eq!("x", _cube);
    }
}
