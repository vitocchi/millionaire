extern crate millionaires_demo;
use millionaires_demo::MillionairesProblem;
fn main() {
    let mut millionaires_problem = MillionairesProblem::new();
    millionaires_problem.add_millionaire(String::from("Bob"),1000000);
    millionaires_problem.add_millionaire(String::from("Alice"),2000000);
    let richest = millionaires_problem.compute_richest();
    println!("Richest millionaire = {}", richest);
}
