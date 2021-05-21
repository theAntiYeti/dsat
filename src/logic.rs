use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Prop(pub bool,pub u32);

#[derive(Debug, Clone)]
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

                outstr.pop(); outstr.pop();
                write!(f,"( {})", outstr)
            },
        }
    }
}

impl Clause {
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
            
        } else { // If already a base true or false, do nothing.
            return new_cl;
        }
    }

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


#[derive(Debug, Clone)]
pub struct Formula {
    pub clauses: Vec<Clause>,
}

#[derive(Debug)]
pub enum Reduced {
    Red(Formula),
    UNSAT,
}

/*
impl fmt::Display for Formula {
    fn fmt(&self, &mut fmt::Formatter) -> fmt::Result {
        
    }
}
*/

impl Formula {
    pub fn mult_assign(&self, props: &Vec<Prop>) -> Reduced {
        let mut new_frm = (*self).clone();

        for i in 0..(new_frm.clauses.len()) {
            new_frm.clauses[i] = new_frm.clauses[i].mult_assign(props);
            if let Clause::F = new_frm.clauses[i] {
                return Reduced::UNSAT;
            }
        }

        return Reduced::Red(new_frm);
        
    }
}
