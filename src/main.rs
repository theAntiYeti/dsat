#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod dpll;
mod logic;
mod serialize;
use dsat::{
    dpll::dpll,
    serialize::{deserialize_formula, serialize_assignment},
};
use std::str::FromStr;

#[post("/solve", data = "<ser_formula>")]
fn solve(ser_formula: String) -> String {
    let repped = ser_formula.replace("%2C", ",").replace("%26","&").replace("%21", "!");
    let split: Vec<&str> = repped.split("=").collect();
    let translated = split[1];
    let formula = deserialize_formula(translated);
    println!("{}", translated);

    match formula {
        Err(_) => return format!("Parse error: {}", translated),
        Ok(frm) => {
            let assignment = dpll(&frm);
            match assignment {
                Some(ass) => return serialize_assignment(&ass),
                None => return String::from_str("UNSAT").unwrap(),
            }
        }
    }
}

fn main() {

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite().attach(cors).mount("/", routes![solve]).launch();
}
