use crate::face::Face;

pub struct CubeState {
    pub F: Face, // Front
    pub B: Face, // Back
    pub L: Face, // Left
    pub R: Face, // Right
    pub U: Face, // Up
    pub D: Face, // Down
}

impl CubeState {
    pub fn new(faces: [Face; 6]) -> Self {
        CubeState {
            F: faces[0],
            B: faces[1],
            L: faces[2],
            R: faces[3],
            U: faces[4],
            D: faces[5],
        }
    }
}
