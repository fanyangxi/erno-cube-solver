// #[path="solver.rs"]
// mod solver;

mod models;
mod solver;

use std::ffi::c_void;
use crate::models::*;
use crate::models::cube_state::CubeState;
use crate::models::misc::FaceRotation;
use crate::models::colors::Color;

fn main() {
    // println!("Hello, world!");
    // solve_it();
    // solver::front_of_house::solve_it();
    // front_of_house::hosting::add_to_waitlist();
}
