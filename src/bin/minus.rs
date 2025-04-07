use lambda_calculus::{
    combinators::Z,
    data::{
        list::pair::{cons, head, is_nil, map, nil, tail, take},
        num::church::{eq, leq, mul, succ},
    },
    *,
};

pub fn minus() -> Term {
    app(
        Z(),
        abs!(
            3, // z, xs, ys
            app!(
                is_nil(),
                Var(2), // xs
                nil(),
                app!(
                    is_nil(),
                    Var(1), // ys
                    Var(2), // return xs
                    app!(
                        leq(),
                        app(head(), Var(2)),
                        app(head(), Var(1)),
                        app!(
                            eq(),
                            app(head(), Var(2)),
                            app(head(), Var(1)),
                            // if x == y => drop x and y
                            app!(Var(3), app(tail(), Var(2)), app(tail(), Var(1))),
                            // if x < y => keep x, recurse
                            app!(
                                cons(),
                                app(head(), Var(2)),
                                app!(Var(3), app(tail(), Var(2)), Var(1))
                            )
                        ),
                        // if x > y => skip y, recurse
                        app!(Var(3), Var(2), app(tail(), Var(1)))
                    )
                )
            )
        ),
    )
}

fn main() {
    // removes all ys from xs

    let xs = vec![
        1.into_church(),
        2.into_church(),
        3.into_church(),
        4.into_church(),
        5.into_church(),
    ]
    .into_pair_list();

    let ys = vec![2.into_church(), 4.into_church()].into_pair_list();

    let expected = vec![1.into_church(), 3.into_church(), 5.into_church()].into_pair_list();

    assert_eq!(
        beta(app!(minus(), xs.clone(), ys.clone()), NOR, 0),
        expected
    );

    // TEST

    fn iterate(func: Term, x: Term) -> Term {
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

    fn from(n: Term) -> Term {
        iterate(succ(), n)
    }

    let first_5_evens = app!(
        take(),
        5.into_church(),
        app!(
            map(),
            app(mul(), 2.into_church()),
            from(1.into_church()) // Generate the list starting from n
        )
    );

    assert_eq!(
        beta(first_5_evens.clone(), NOR, 0),
        vec![
            2.into_church(),
            4.into_church(),
            6.into_church(),
            8.into_church(),
            10.into_church(),
        ]
        .into_pair_list()
    );

    assert_eq!(
        beta(app!(minus(), xs.clone(), first_5_evens), NOR, 0),
        vec![1.into_church(), 3.into_church(), 5.into_church()].into_pair_list()
    );
}
