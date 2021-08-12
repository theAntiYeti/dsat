#[macro_use]
extern crate serial_test;

use dsat::logic;
use dsat::dpll;
use dsat::serialize;

/* Passing tests */
#[test]
#[serial]
fn test_solver1() {
    println!("===== Integraion test 1. =====");
    let encoded_formula = "1,2,3&!2,4&!3,4";
    println!("Formula serialized: {}", encoded_formula);
    let formula = serialize::deserialize_formula(encoded_formula).unwrap();
    println!("Formula deserialized: {:?}", formula);
    let solution = dpll::dpll(&formula).unwrap();
    println!("Solution: {:?}", solution);
    
    let expected: Vec<logic::Clause> = vec![];
    assert_eq!(formula.mult_assign(&solution), logic::Reduced::Red(logic::Formula { clauses: expected}));
}

#[test]
#[serial]
fn test_solver2() {
    println!("===== Integraion test 2. =====");
    let encoded_formula = "1,2,!3&3,4,!5&!2,!1,7&!7,4,5";
    println!("Formula serialized: {}", encoded_formula);
    let formula = serialize::deserialize_formula(encoded_formula).unwrap();
    println!("Formula deserialized: {:?}", formula);
    let solution = dpll::dpll(&formula).unwrap();
    println!("Solution: {:?}", solution);
    
    let expected: Vec<logic::Clause> = vec![];
    assert_eq!(formula.mult_assign(&solution), logic::Reduced::Red(logic::Formula { clauses: expected}));
}


/* Test UNSAT */
#[test]
#[serial]
fn test_solver_f1() {
    let encoded_formula = "1&!1";
    let formula = serialize::deserialize_formula(encoded_formula).unwrap();
    assert!(dpll::dpll(&formula).is_none());
}
