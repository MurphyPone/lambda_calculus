use lambda_calculus::{
    combinators::Z,
    data::{
        list::pair::{cons, head, is_nil, nil, tail},
        num::church::leq,
    },
    *,
};

fn main() {
    // [0, 2, 3]
    let list = || vec![0.into_church(), 2.into_church(), 3.into_church()].into_pair_list();

    pub fn insert_sorted(x: Term, xs: Term) -> Term {
        let recursive = app(
            Z(),
            abs!(3, {
                let f = Var(3); // recursive function
                let x = Var(2); // the value to insert
                let xs = Var(1); // the list

                // is_nil(xs)
                let base_case = app!(cons(), x.clone(), nil());

                // leq x (head xs)
                let insert_here = app!(cons(), x.clone(), xs.clone());

                // recursive case
                let recurse = app!(
                    cons(),
                    app!(head(), xs.clone()),
                    app!(f.clone(), x.clone(), app!(tail(), xs.clone()))
                );

                app!(
                    is_nil(),
                    xs.clone(),
                    base_case,
                    app!(
                        leq(),
                        x.clone(),
                        app!(head(), xs.clone()),
                        insert_here,
                        recurse
                    )
                )
            }),
        );

        app!(recursive, x, xs)
    }

    assert_eq!(
        beta(insert_sorted(1.into_church(), list()), NOR, 0),
        vec![
            0.into_church(),
            1.into_church(),
            2.into_church(),
            3.into_church()
        ]
        .into_pair_list()
    );

    let mut ex = insert_sorted(1.into_church(), list());

    let mut t = 0;
    while ex.reduce(NOR, 1) != 0 {
        println!();
        println!("{}: {}", t, ex);
        t += 1;
    }
}
