use lambda_calculus::{
    combinators::Z,
    data::{
        list::pair::{cons, head, is_nil, nil, tail},
        num::church::leq,
    },
    *,
};

pub fn merge() -> Term {
    app(
        Z(),
        abs!(
            3, // merge = λz xs ys ...
            app!(
                is_nil(),
                Var(2), // xs
                Var(1), // ys
                app!(
                    is_nil(),
                    Var(1), // ys
                    Var(2), // xs
                    app!(
                        leq(),
                        app(head(), Var(2)),
                        app(head(), Var(1)),
                        app!(
                            cons(),
                            app(head(), Var(2)),
                            app!(Var(3), app(tail(), Var(2)), Var(1))
                        ),
                        app!(
                            cons(),
                            app(head(), Var(1)),
                            app!(Var(3), Var(2), app(tail(), Var(1)))
                        )
                    )
                )
            )
        ),
    )
}

pub fn merge_named() -> Term {
    app(
        Z(),
        abs!(3, {// merge = λz xs ys ...
            let recurse = Var(3);
            let xs = Var(2);
            let ys = Var(1);
            app!(
                is_nil(),
                xs.clone(), // xs
                ys.clone(), // ys
                app!(
                    is_nil(),
                    ys.clone(), // ys
                    xs.clone(), // xs
                    app!(
                        leq(),
                        app(head(), xs.clone()),
                        app(head(), ys.clone()),
                        app!(
                            cons(),
                            app(head(), xs.clone()),
                            app!(recurse.clone(), app(tail(), xs.clone()), ys.clone())
                        ),
                        app!(
                            cons(),
                            app(head(), ys.clone()),
                            app!(recurse, xs.clone(), app(tail(), ys.clone()))
                        )
                    )
                )
            )
        }),
    )
}

fn main() {
    let xs = vec![1.into_church()].into_pair_list();

    let ys = vec![0.into_church(), 2.into_church()].into_pair_list();

    assert_eq!(
        beta(app!(merge(), xs.clone(), ys.clone()), NOR, 0),
        vec![0.into_church(), 1.into_church(), 2.into_church(),].into_pair_list()
    );

    assert_eq!(
        beta(
            app!(merge(), nil(), vec![1.into_church()].into_pair_list()),
            NOR,
            0
        ),
        vec![1.into_church(),].into_pair_list()
    );

    // named variant 

    assert_eq!(
        beta(app!(merge_named(), xs.clone(), ys.clone()), NOR, 0),
        vec![0.into_church(), 1.into_church(), 2.into_church(),].into_pair_list()
    );

    assert_eq!(
        beta(
            app!(merge_named(), nil(), vec![1.into_church()].into_pair_list()),
            NOR,
            0
        ),
        vec![1.into_church(),].into_pair_list()
    );
}
