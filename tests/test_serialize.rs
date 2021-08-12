use std::str::FromStr;

use tokio_test::assert_err;

use dsat::logic;
use logic::Prop;
use dsat::serialize;

/* Test serialize_prop */
#[test]
fn test_serialize_prop() {
    let prop1 = Prop(true, 1);
    let prop2 = Prop(false, 2);

    assert_eq!(serialize::serialize_prop(&prop1), "1");
    assert_eq!(serialize::serialize_prop(&prop2), "!2");
}

/* Test deserialize_prop */
#[test]
fn test_deserialize_prop_success() {
    let str1 = "45";
    let str2 = "!17";
    assert_eq!(serialize::deserialize_prop(str1).unwrap(), Prop(true, 45));
    assert_eq!(serialize::deserialize_prop(str2).unwrap(), Prop(false, 17));
}

#[test]
fn test_deserialize_prop_error() {
    let str1 = "0ab";
    let error = serialize::deserialize_prop(str1);
    assert_err!(error);
}

/* Test serialize_assignment */
#[test]
fn test_serialize_assignment() {
    let clause = vec![Prop(true, 2), Prop(false, 70), Prop(false, 29), Prop(true,15)];
    let expected = String::from_str("2,!70,!29,15").unwrap();
    assert_eq!(serialize::serialize_assignment(&clause),expected);
}
