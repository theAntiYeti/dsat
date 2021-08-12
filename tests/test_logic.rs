use dsat::logic;

/* Test Clause */
#[test]
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
    let test_clause = logic::Clause::Vars(vec![logic::Prop(false, 1), logic::Prop(false, 2)]);

    let assigned_1 = test_clause.assign(logic::Prop(true, 1));
    assert_eq!(assigned_1, logic::Clause::Vars(vec![logic::Prop(false, 2)]));
    let assigned_2 = assigned_1.assign(logic::Prop(true, 2));
    assert_eq!(assigned_2, logic::Clause::F);
}

#[test]
fn test_clause_mult_assign() {
    let test_clause = logic::Clause::Vars(vec![logic::Prop(false, 1), logic::Prop(false, 2)]);
    let assigned = test_clause.mult_assign(&vec![logic::Prop(true, 1), logic::Prop(true, 2)]);

    assert_eq!(assigned, logic::Clause::F);
}

/* Test Formula */
#[test]
fn test_formula_mult_assign() {
    let test_formula = logic::Formula {
        clauses: vec![
            logic::Clause::Vars(vec![
                logic::Prop(false, 1),
                logic::Prop(false, 2),
                logic::Prop(true, 3),
            ]),
            logic::Clause::Vars(vec![logic::Prop(true, 1)]),
        ],
    };

    let assigned = test_formula.mult_assign(&vec![logic::Prop(true, 1), logic::Prop(true, 2)]);
    assert_eq!(
        assigned,
        logic::Reduced::Red(logic::Formula {
            clauses: vec![
                logic::Clause::Vars(vec![logic::Prop(true, 3)]),
                logic::Clause::T
            ]
        })
    );
}

#[test]
fn test_formula_mult_assign_false() {
    let test_formula = logic::Formula {
        clauses: vec![
            logic::Clause::Vars(vec![
                logic::Prop(false, 1),
                logic::Prop(false, 2),
                logic::Prop(true, 3),
            ]),
            logic::Clause::Vars(vec![logic::Prop(true, 1)]),
        ],
    };

    let assigned = test_formula.mult_assign(&vec![logic::Prop(false, 1)]);
    assert_eq!(assigned, logic::Reduced::UNSAT);
}

#[test]
fn test_get_unit() {
    let test_formula = logic::Formula {
        clauses: vec![
            logic::Clause::Vars(vec![
                logic::Prop(false, 1),
                logic::Prop(false, 2),
                logic::Prop(true, 3),
            ]),
            logic::Clause::Vars(vec![logic::Prop(true, 1)]),
        ],
    };

    let unit_option = test_formula.get_unit();

    assert_eq!(unit_option, Some(logic::Prop(true, 1)));
}
