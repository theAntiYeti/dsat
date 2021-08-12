use crate::logic;
use logic::Clause;
use logic::Prop;

pub fn serialize_prop(prop: &Prop) -> String {
    let ex_indicator = if prop.0 { "" } else { "!" };
    format!("{}{}", ex_indicator, prop.1)
}

pub fn deserialize_prop(txt: &str) -> Result<Prop, Box<dyn std::error::Error>> {
    let arity = txt.chars().next() != Some('!');
    let st: usize = if arity { 0 } else { 1 };
    let value = txt[st..].parse::<u32>()?;

    Ok(Prop(arity, value))
}

pub fn serialize_assignment(ass: &Vec<Prop>) -> String {
    let serialised_entries: Vec<String> = ass.iter().map(|x| serialize_prop(x)).collect();
    serialised_entries.join(",")
}

pub fn deserialize_clause(txt: &str) -> Result<Clause, Box<dyn std::error::Error>> {
    let split = txt.split(",");
    let mut prop_list: Vec<Prop> = Vec::new();

    for part in split {
        prop_list.push(deserialize_prop(part)?);
    }

    Ok(Clause::Vars(prop_list))
}
