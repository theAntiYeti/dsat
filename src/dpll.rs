use crate::logic;
use logic::Clause;
use logic::Formula;
use logic::Prop;
use logic::Reduced;

#[derive(Debug)]
pub enum Marker {
    U, // Unit Propagate
    T, // Decide True
    F, // Decide False
}

pub fn dpll(phi: &Formula) -> Option<Vec<Prop>> {
    let mut assign: Vec<Prop> = Vec::new(); // The vector representing the assignment
    let mut history: Vec<Marker> = Vec::new(); // The marker history, for backtracking

    let mut partial = (*phi).clone();

    // Unit Props. Don't assign to history.
    loop {
        let unit = partial.get_unit();
        match unit {
            None => break,
            Some(prop) => {
                assign.push(prop);
                let result = partial.mult_assign(&vec![prop]);

                match result {
                    Reduced::UNSAT => return None, // If all we've done is unit propagate this has no chance.
                    Reduced::Red(form) => partial = form,
                }
            }
        }
    }

    while {
        // Check for completion
        if partial.is_true() {
            return Some(assign);
        }

        // Do an assignment, check if can unit prop
        let unit = partial.get_unit();
        let mut result = Reduced::UNSAT; // Placeholder

        match unit {
            Some(prop) => {
                assign.push(prop);
                history.push(Marker::U);
                result = partial.mult_assign(&vec![prop]);
            }
            None => {
                let var = partial.get_var().unwrap();
                assign.push(Prop(true, var));
                history.push(Marker::T);
                result = partial.mult_assign(&vec![Prop(true, var)]);
                //println!("{:?}", result);
            }
        }
        // While UNSAT, we backtrack
        while let Reduced::UNSAT = result {
            while history.len() > 0 {
                let mark = history.pop().unwrap();
                let prop = assign.pop().unwrap();
                if let Marker::T = mark {
                    history.push(Marker::F);
                    assign.push(Prop(false, prop.1));
                    result = phi.mult_assign(&assign);
                }
            }
        }

        match result {
            Reduced::UNSAT => panic!("result should not be unsat!"),
            Reduced::Red(form) => partial = form,
        }

        // Condition
        history.len() > 0
    } {}

    return None;
}
