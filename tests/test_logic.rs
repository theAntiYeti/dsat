use dsat;
use dsat::logic;

#[test]
/* Test Clause */
fn test_clause_assign() {
    let test_clause = logic::Clause::Vars(vec![
        logic::Prop(true, 1),
        logic::Prop(true, 2),
        logic::Prop(false, 3),
    ]);
    let assigned = test_clause.assign(logic::Prop(true, 1));
    assert_eq!(assigned, logic::Clause::T);
}
