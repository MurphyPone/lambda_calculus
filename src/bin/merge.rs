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

fn main() {
    let xs = vec![1.into_church()].into_pair_list();

    let ys = vec![0.into_church(), 2.into_church()].into_pair_list();

    assert_eq!(
        beta(app!(merge(), xs, ys), NOR, 0),
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
}
