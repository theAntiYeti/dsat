use crate::logic;
use logic::Clause;
use logic::Prop;

/// Serializes a proposition as a String.
///
/// Returns parity indicator '!' and numerical index of Proposition.
///
/// # Arguments
///
/// * `prop` - Proposition to serialize.
pub fn serialize_prop(prop: &Prop) -> String {
    let ex_indicator = if prop.0 { "" } else { "!" };
    format!("{}{}", ex_indicator, prop.1)
}

/// Takes a string representation and parses as a Prop.
///
/// # Arguments
///
/// * `txt` - String to deserialize.
pub fn deserialize_prop(txt: &str) -> Result<Prop, Box<dyn std::error::Error>> {
    let arity = txt.chars().next() != Some('!');
    let st: usize = if arity { 0 } else { 1 };
    let value = txt[st..].parse::<u32>()?;

    Ok(Prop(arity, value))
}

/// Takes an assignment (list of propositions) and serializes.
///
/// # Arguments
///
/// * `ass` - Vector representing the assignment.
pub fn serialize_assignment(ass: &Vec<Prop>) -> String {
    let serialised_entries: Vec<String> = ass.iter().map(|x| serialize_prop(x)).collect();
    serialised_entries.join(",")
}

/// Takes a serialized clause and attempts to parse as a Clause.
///
/// # Arguments
///
/// * `txt` - String to deserialize.
pub fn deserialize_clause(txt: &str) -> Result<Clause, Box<dyn std::error::Error>> {
    let split = txt.split(",");
    let mut prop_list: Vec<Prop> = Vec::new();

    for part in split {
        prop_list.push(deserialize_prop(part)?);
    }

    Ok(Clause::Vars(prop_list))
}
