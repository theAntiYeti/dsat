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

