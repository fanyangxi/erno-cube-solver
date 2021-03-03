#[path="../src/solver.rs"]
mod solver;

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
    let result = solver::front_of_house::add_to_waitlist();
    assert_eq!(result, 4);
}