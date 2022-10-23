use std::collections::HashMap;
fn main() {
    let teams = vec![String::from("hello"),String::from("world")];
    let initial_scores = vec![10,49];
    let scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
}
