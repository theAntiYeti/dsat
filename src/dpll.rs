mod logic;
use logic::Clause;
use logic::Prop;
use logic::Formula;

pub enum Marker {
    U, // Unit Propagate
    T, // Decide True
    F, // Decide False
}

pub fn dpll(&phi: Formula) -> Option<Vec<Prop>> {
    let mut assign : Vec<Prop> = Vec::new(); // The vector representing the assignment
    let mut history : Vec<Marker> = Vec::new(); // The marker history, for backtracking

    let mut partial = (*phi).clone();

    // Push the first literal to the assign queue.
    let var = partial.get_var();
    
    if let Some(name) = var {
        assign.push(Prop(true, name));
        history.push(T);
        partial = partial.mult_assign(&assign);
    } // if var == None then the next while loop will be skipped completely.

    while partial.len() > 0 {
        // Check for completion


        // Check for UNSAT and backtrack
        

        // Make
    }

    return None;
}
