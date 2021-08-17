#![feature(decl_macro)]

use std::io::Read;

#[macro_use]
extern crate rocket;
use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status, ContentType};

mod dpll;
mod logic;
mod serialize;
use dsat::{
    dpll::dpll,
    serialize::{deserialize_formula, serialize_assignment},
};
use std::str::FromStr;

const LIMIT: u64 = 1000000;

struct StringWrapper {
    string: String,
}

impl FromDataSimple for StringWrapper {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        return Success(StringWrapper{ string });
    }
}

#[post("/solve", data = "<ser_formula>")]
fn solve(ser_formula: StringWrapper) -> String {
    let string = ser_formula.string;
    let repped = string.replace("%2C", ",").replace("%26","&").replace("%21", "!");
    let split: Vec<&str> = repped.split("=").collect();
    let translated = split[1];
    let formula = deserialize_formula(translated);

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
