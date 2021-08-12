use crate::logic;
use logic::Prop;

pub fn serialize_prop(prop: &Prop) -> String {
   let ex_indicator = if prop.0 { "" } else { "!" };
    format!("{}{}", ex_indicator, prop.1)
}
