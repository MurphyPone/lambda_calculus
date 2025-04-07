use lambda_calculus::{
    combinators::Z,
    data::{
        list::pair::{cons, nil, take},
        num::church::succ,
    },
    *,
};

pub fn iterate(func: Term, x: Term) -> Term {
    let recursive = app(
        Z(),
        abs!(3, {
            let f = Var(3); // recursive function (Var(3) is Z’s parameter)
            let func = Var(2); // function to apply (Var(2) is the func parameter)
            let x = Var(1); // current value (Var(1) is the x parameter)

            // Cons the current value and recursively apply func to the next value
            app!(
                cons(),
                x.clone(),
                app!(f.clone(), func.clone(), app!(func.clone(), x))
            )
        }),
    );

    // Apply recursive function to func and x
    app!(recursive, func, x)
}

pub fn from(n: Term) -> Term {
    iterate(succ(), n)
}

fn main() {
    assert_eq!(
        beta(app!(take(), 2.into_church(), from(0.into_church())), NOR, 0),
        vec![0.into_church(), 1.into_church()].into_pair_list()
    );

    assert_eq!(
        beta(app!(take(), 0.into_church(), from(0.into_church())), NOR, 0),
        vec![].into_pair_list()
    );

    assert_eq!(
        beta(app!(take(), 3.into_church(), from(0.into_church())), NOR, 0),
        vec![0.into_church(), 1.into_church(), 2.into_church()].into_pair_list()
    );

    let mut ex = app!(take(), 2.into_church(), from(0.into_church()));

    let mut t = 0;
    while ex.reduce(NOR, 1) != 0 {
        println!();
        println!("{}: {}", t, ex);
        t += 1;
    }

    println!(
        "(cons 0 (cons 1 nil))\n   = {}",
        beta(
            app!(
                cons(),
                0.into_church(),
                app!(cons(), 1.into_church(), nil())
            ),
            NOR,
            4
        )
    )
}
