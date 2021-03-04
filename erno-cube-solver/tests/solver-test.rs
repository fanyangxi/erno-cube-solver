#[path="../src/solver.rs"]
mod solver;

#[path="../src/models.rs"]
mod models;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = solver::front_of_house::add_to_waitlist();
//         assert_eq!(result, 4);
//     }
// }


#[test]
fn it_works() {
    let state = models::CubeState {
        F: models::Face(),
        B: models::Face(), // Back
        L: models::Face(), // Left
        R: models::Face(), // Right
        U: models::Face(), // Up
        D: models::Face(), // Down
    };
    let result = solver::front_of_house::solve_it(state);
    assert_eq!(result, 4);
}