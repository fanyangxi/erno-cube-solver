use crate::models::colors::Color;

// For each face, top-left piece is 0,0:
#[derive(Debug, Copy, Clone)]
pub struct Face {
    pub p00: Color, // Corner
    pub p01: Color, // Edge
    pub p02: Color, // Corner
    pub p10: Color, // Edge
    pub p11: Color, // *Center*
    pub p12: Color, // Edge
    pub p20: Color, // Corner
    pub p21: Color, // Edge
    pub p22: Color, // Corner
}

impl Face {
    pub fn new(items: [[Color; 3]; 3]) -> Self {
        Face {
            p00: items[0][0],
            p01: items[0][1],
            p02: items[0][2],
            p10: items[1][0],
            p11: items[1][1],
            p12: items[1][2],
            p20: items[2][0],
            p21: items[2][1],
            p22: items[2][2],
        }
    }
}
