pub mod CrossSolver {
    use std::collections::HashMap;

    pub fn steps() -> HashMap<String, HashMap<String, Vec<String>>> {
        // White-Cross-Solver
        // This class solves the white cross on the down face
        let steps: HashMap<String, HashMap<String, Vec<String>>> = [
            ("U".into(), [
                ("R".into(), vec![]),
                ("L".into(), vec![]),
                ("F".into(), vec![]),
                ("B".into(), vec![]),
            ].iter().cloned().collect()),
            ("D".into(), [
                ("R".into(), ["R2"].iter().map(|&x| x.into()).collect()),
                ("L".into(), ["L2"].iter().map(|&x| x.into()).collect()),
                ("F".into(), ["F2"].iter().map(|&x| x.into()).collect()),
                ("B".into(), ["B2"].iter().map(|&x| x.into()).collect()),
            ].iter().cloned().collect()),
            ("F".into(), [
                ("U".into(), ["F", "R", "U'", "R'", "F'"].iter().map(|&x| x.into()).collect()),
                ("D".into(), ["F'", "R", "U'", "R'"].iter().map(|&x| x.into()).collect()),
                ("R".into(), ["R", "U", "R'"].iter().map(|&x| x.into()).collect()),
                ("L".into(), ["L'", "U'", "L"].iter().map(|&x| x.into()).collect()),
            ].iter().cloned().collect()),
            ("B".into(), [
                ("U".into(), ["B", "L", "U'", "L'", "B'"].iter().map(|&x| x.into()).collect()),
                ("D".into(), ["B", "R'", "U", "R"].iter().map(|&x| x.into()).collect()),
                ("R".into(), ["R'", "U", "R"].iter().map(|&x| x.into()).collect()),
                ("L".into(), ["L", "U'", "L'"].iter().map(|&x| x.into()).collect()),
            ].iter().cloned().collect()),
            ("L".into(), [
                ("U".into(), ["L", "F", "U'", "F'", "L'"].iter().map(|&x| x.into()).collect()),
                ("D".into(), ["L'", "F", "U'", "F'"].iter().map(|&x| x.into()).collect()),
                ("F".into(), ["F", "U'", "F'"].iter().map(|&x| x.into()).collect()),
                ("B".into(), ["B'", "U", "B"].iter().map(|&x| x.into()).collect()),
            ].iter().cloned().collect()),
            ("R".into(), [
                ("U".into(), ["R'", "F'", "U", "F", "R"].iter().map(|&x| x.into()).collect()),
                ("D".into(), ["R", "F'", "U", "F"].iter().map(|&x| x.into()).collect()),
                ("F".into(), ["F'", "U", "F"].iter().map(|&x| x.into()).collect()),
                ("B".into(), ["B", "U'", "B'"].iter().map(|&x| x.into()).collect()),
            ].iter().cloned().collect()),
        ].iter().cloned().collect();
        return steps;
    }

    pub fn first_step(white_facing: String, color_facing: String) {
        let the_steps = steps();
        let color_faces = match the_steps.get(&white_facing) {
            Some(v) => v,
            None => panic!("Failed to find in the_steps {}.", &white_facing)
        };
        let result = match color_faces.get(&color_facing) {
            Some(v) => v,
            None => panic!("Failed to find in color_faces {}.", &color_facing)
        };
        println!("{:#?} days", result);
    }
}
