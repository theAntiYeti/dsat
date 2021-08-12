use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents a variable and whether it's true or false.
pub struct Prop(pub bool, pub u32);

#[derive(Debug, Clone, PartialEq)]
/// Represents a clause of variables disjuncted together.
pub enum Clause {
    Vars(Vec<Prop>),
    T,
    F,
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (*self).clone() {
            Clause::T => write!(f, "T"),
            Clause::F => write!(f, "F"),
            Clause::Vars(vs) => {
                let mut outstr = String::with_capacity(2 * vs.len());

                for &Prop(val, var) in vs.iter() {
                    if !val {
                        outstr.push_str("~");
                    }

                    outstr.push_str(&var.to_string());
                    outstr.push_str(" V ");
                }

                outstr.pop();
                outstr.pop();
                write!(f, "( {})", outstr)
            }
        }
    }
}

impl Clause {
    /// Performs variable assignment and returns a clause representing the result.
    ///
    /// # Arguments
    ///
    /// * `ass` - Prop representing the variable to assign and what value to give.
    pub fn assign(&self, ass: Prop) -> Clause {
        let new_cl = (*self).clone();

        if let Clause::Vars(mut vs) = new_cl {
            // Remove values set to false by idempotence.
            vs.retain(|&Prop(val, name)| !(val != ass.0) || !(name == ass.1));

            if vs.is_empty() {
                return Clause::F;
            } else if vs.contains(&ass) {
                return Clause::T;
            } else {
                return Clause::Vars(vs);
            }
        } else {
            // If already a base true or false, do nothing.
            return new_cl;
        }
    }

    /// Takes a partial assignment and applies to clause returning result.
    ///
    /// # Arguments
    ///
    /// * `props` - Vector of Prop representing partial assignment.
    pub fn mult_assign(&self, props: &Vec<Prop>) -> Clause {
        let mut red_step = (*self).clone();
        for prp in (*props).iter() {
            red_step = red_step.assign(*prp);
            if let Clause::Vars(_) = red_step {
                continue;
            } else {
                return red_step;
            }
        }

        return red_step;
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a formula (in CNF) as a list of clauses.
pub struct Formula {
    pub clauses: Vec<Clause>,
}

#[derive(Debug, Clone, PartialEq)]
/// A more lucid Option wrapper for dealing with operations on formulae.
pub enum Reduced {
    Red(Formula),
    UNSAT,
}

impl Formula {
    /// Returns result of assigning props to self.
    ///
    /// Returns UNSAT if any clause evaluated to F.
    ///
    /// # Arguments
    ///
    /// * `props` - Vector of Prop representing partial assignment.
    pub fn mult_assign(&self, props: &Vec<Prop>) -> Reduced {
        let mut new_clauses: Vec<Clause> = Vec::with_capacity(self.clauses.len());

        for i in 0..(self.clauses.len()) {
            let new_cl = self.clauses[i].mult_assign(props);

            // F represents unsatisfiable (all need be true in SAT), T can be
            // ommitted as the empty conjunction is defined to be true.
            match new_cl {
                Clause::F => return Reduced::UNSAT,
                Clause::T => continue,
                _ => new_clauses.push(new_cl),
            }
        }

        return Reduced::Red(Formula {
            clauses: new_clauses,
        });
    }

    /// Returns a variable still represented in the formula.
    ///
    /// In the context of the algorithms *should* be guaranteed
    /// to return something not yet assigned.
    pub fn get_var(&self) -> Option<u32> {
        for cls in (*self).clauses.iter() {
            if let Clause::Vars(props) = cls {
                for Prop(_val, name) in (*props).iter() {
                    return Some(*name);
                }
            }
        }
        return None;
    }

    /// Returns a unit clause, if present, for unit propagation.
    pub fn get_unit(&self) -> Option<Prop> {
        for cls in (*self).clauses.iter() {
            if let Clause::Vars(props) = cls {
                if props.len() == 1 {
                    return Some(props[0]);
                }
            }
        }
        return None;
    }

    /// Returns true if clauses empty (empty disjunction) or all true.
    pub fn is_true(&self) -> bool {
        for clause in self.clauses.iter() {
            if let Clause::T = clause {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }
}
