// #[path="../src/main.rs"]
// mod main;
// use crate::main::*;
//
// // #[path="../src/models.rs"]
// // mod models;
// use crate::models::*;

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
    let state = CubeState {
        F: Face(),
        B: Face(), // Back
        L: Face(), // Left
        R: Face(), // Right
        U: Face(), // Up
        D: Face(), // Down
    };
    let result = solve_it(state);
    assert_eq!(result, 4);
}
