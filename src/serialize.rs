use crate::logic;
use logic::Prop;

pub fn serialize_prop(prop: &Prop) -> String {
   let ex_indicator = if prop.0 { "" } else { "!" };
    format!("{}{}", ex_indicator, prop.1)
}

pub fn deserialize_prop(txt: &str) -> Result<Prop, Box<dyn std::error::Error>> {
    let arity = txt.chars().next() != Some('!');
    let st: usize = if arity {0} else {1};
    let value = txt[st..].parse::<u32>()?;
    Ok(Prop(arity, value))
}
