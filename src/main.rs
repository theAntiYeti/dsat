#[macro_use] extern crate rocket;

mod dpll;
use dsat::{dpll::dpll, serialize::{deserialize_formula, serialize_assignment}};
use std::str::FromStr;

#[post("/solve", data = "<ser_formula>")]
fn solve(ser_formula: &str) -> String {
    let formula = deserialize_formula(ser_formula);

    match formula {
        Err(_) => return String::from_str("Parse error!").unwrap(),
        Ok(frm) => {
            let assignment = dpll(&frm);
            match assignment {
                Some(ass) => return serialize_assignment(&ass),
                None => return String::from_str("UNSAT").unwrap(),
            }
        },
    }
}



#[launch]
fn launch() -> _ {
   rocket::build().mount("/", routes![solve]) 
}
