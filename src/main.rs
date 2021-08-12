mod logic;
use logic::Clause;
use logic::Formula;
use logic::Prop;

mod dpll;

fn main() {
    let a_or_b = Clause::Vars(vec![Prop(false, 1), Prop(true, 2)]);
    let a_or_c = Clause::Vars(vec![Prop(false, 1), Prop(true, 3)]);

    let form = Formula {
        clauses: (vec![a_or_b, a_or_c]),
    };
    println!("{:?}", form);

    //let res = logic::assign(&a_or_b, 1, true);
    let res = form.mult_assign(&vec![Prop(false, 1)]);
    println!("{:?}", res);

    let stuff = dpll::dpll(&form);

    println!("{:?}", stuff);
}
