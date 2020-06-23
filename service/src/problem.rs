use lp_modeler::dsl::*;
use lp_modeler::solvers::{GlpkSolver, SolverTrait, Status};
use std::collections::HashMap;

pub fn run_problem() -> (Status, HashMap<String, f32>) {
    let cats = vec!["A", "B", "C"];
    let dogs = vec!["D", "E", "F"];
    let compatibility_score: HashMap<(&str, &str), f32> = vec![
        (("A", "D"), 50.0),
        (("A", "E"), 75.0),
        (("A", "F"), 75.0),
        (("B", "D"), 60.0),
        (("B", "E"), 95.0),
        (("B", "F"), 80.0),
        (("C", "D"), 60.0),
        (("C", "E"), 70.0),
        (("C", "F"), 80.0),
    ]
    .into_iter()
    .collect();

    // Define Problem
    let mut problem = LpProblem::new("Matchmaking", LpObjective::Maximize);

    // Define Variables
    let vars: HashMap<(&str, &str), LpBinary> = cats
        .iter()
        .flat_map(|&m| {
            dogs.iter().map(move |&w| {
                let key = (m, w);
                let value = LpBinary::new(&format!("{}_{}", m, w));
                (key, value)
            })
        })
        .collect();

    // Define Objective Function
    let obj_vec: Vec<LpExpression> = {
        vars.iter().map(|(&(m, w), bin)| {
            let &coef = compatibility_score.get(&(m, w)).unwrap();
            coef * bin
        })
    }
    .collect();
    problem += obj_vec.sum();

    // Define Constraints
    // - constraint 1: Each cat must be assigned to exactly one dog
    for &c in &cats {
        problem += sum(&dogs, |&d| vars.get(&(c, d)).unwrap()).equal(1);
    }

    // - constraint 2: Each dog must be assigned to exactly one cat
    for &d in &dogs {
        problem += sum(&cats, |&c| vars.get(&(c, d)).unwrap()).equal(1);
    }

    // Run Solver
    let solver = GlpkSolver::new();
    let result = solver.run(&problem);

    // Compute final objective function value
    // (terminate if error, or assign status & variable values)
    assert!(result.is_ok(), result.unwrap_err());
    result.unwrap()
}
