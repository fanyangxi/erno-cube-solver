// #[path="models.rs"]
// mod models;

// use crate::models;
// use crate::src::models;

// use super::models;

pub mod solver {
    use crate::models::cube_state::CubeState;
    use crate::models::misc::FaceRotation;

    pub fn solve_it(_state: CubeState) -> Vec<FaceRotation> {
        return vec![];
    }
}

#[cfg(test)]
mod tests {
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

        let cube_state = CubeState {
            front: face1,
            back: face2,
            left: face3,
            right: face4,
            up: face5,
            down: face6,
        };
        let _results = solve_it(cube_state);
        assert_eq!(2 + 2, 4);
    }
}

// pub fn solve_it() {
//     println!(">> Hello, world!");
// }

// mod front_of_house {
//     fn add_to_waitlist() {}
// }
