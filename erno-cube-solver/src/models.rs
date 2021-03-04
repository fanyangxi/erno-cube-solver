// Cublet
// Face (6),
// Center-Piece (1), 
// Edge-Piece (4), 
// Corner-Piece (4),

pub struct CubeState {
    F: Face, // Front
    B: Face, // Back
    L: Face, // Left
    R: Face, // Right
    U: Face, // Up
    D: Face, // Down
}

pub struct Face {
    p11: Colors, // Corner
    p12: Colors, // Edge
    p13: Colors, // Corner
    p21: Colors, // Edge
    p22: Colors, // *Center*
    p23: Colors, // Edge
    p31: Colors, // Corner
    p32: Colors, // Edge
    p33: Colors, // Corner
}

pub enum Colors {
    Red,
    Green,
    Oragne,
    Blue,
    //
    Yellow,
    White,
}

pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

pub struct FaceRotation {
    target: Face,
    direction: RotationDirection,
    turns: i32,
}
