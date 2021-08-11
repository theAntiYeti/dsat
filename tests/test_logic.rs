use dsat;
use dsat::logic;

#[test]
/* Test Clause */
fn test_clause_assign_true() {
    let test_clause = logic::Clause::Vars(vec![
        logic::Prop(true, 1),
        logic::Prop(true, 2),
        logic::Prop(false, 3),
    ]);
    let assigned = test_clause.assign(logic::Prop(true, 1));
    assert_eq!(assigned, logic::Clause::T);
}

#[test]
fn test_clause_assign_false() {
    let test_clause = logic::Clause::Vars(vec![
        logic::Prop(false,1),
        logic::Prop(false,2),
    ]);
    let assigned_1 = test_clause.assign(logic::Prop(true,1));
    assert_eq!(assigned_1, logic::Clause::Vars(vec![logic::Prop(false, 2)]));
    let assigned_2 = assigned_1.assign(logic::Prop(true,2));
    assert_eq!(assigned_2, logic::Clause::F);
}
