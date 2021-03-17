use crate::models::face::Face;

pub struct CubeState {
    pub front: Face, // Front
    pub back: Face, // Back
    pub left: Face, // Left
    pub right: Face, // Right
    pub up: Face, // Up
    pub down: Face, // Down
}

impl CubeState {
    pub fn new(faces: [Face; 6]) -> Self {
        CubeState {
            front: faces[0],
            back: faces[1],
            left: faces[2],
            right: faces[3],
            up: faces[4],
            down: faces[5],
        }
    }
}
