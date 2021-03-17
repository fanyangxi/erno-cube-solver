// #[path="models.rs"]
// mod models;

// use crate::models;
// use crate::src::models;

// use super::models;

pub mod solver {
    use crate::models::cube_state::CubeState;
    use crate::models::misc::FaceRotation;
    use crate::models::colors::Color;

    pub fn solve_it(state: CubeState) -> Vec<FaceRotation> {
        let aaa = Color::Red;

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use crate::solve_it;
    use crate::models::cube_state::CubeState;
    use crate::models::face::Face;
    use crate::models::colors::Color;
    use crate::solver::solver::solve_it;

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

        let cubeState = CubeState {
            F: face1,
            B: face2,
            L: face3,
            R: face4,
            U: face5,
            D: face6,
        };
        let results = solve_it(cubeState);
        assert_eq!(2 + 2, 4);
    }
}

// pub fn solve_it() {
//     println!(">> Hello, world!");
// }

// mod front_of_house {
//     fn add_to_waitlist() {}
// }
