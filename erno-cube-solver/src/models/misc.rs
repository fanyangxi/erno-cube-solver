// Cublet
// Face (6),
// Center-Piece (1),
// Edge-Piece (4),
// Corner-Piece (4),

use crate::models::face::Face;

pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

pub struct FaceRotation {
    target: Face,
    direction: RotationDirection,
    turns: i32,
}
