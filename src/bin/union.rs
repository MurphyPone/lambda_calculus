use lambda_calculus::data::boolean::if_else;
use lambda_calculus::data::num::church::lt;
use lambda_calculus::{
    combinators::Z,
    data::list::pair::{cons, head, is_nil, nil, tail},
    *,
};

/// UNION = Z (λf xs ys.
///   IF (IS_NIL xs)
///     THEN ys
///     ELSE IF (CONTAINS ys (HEAD xs))
///       THEN f (TAIL xs) ys
///       ELSE CONS (HEAD xs) (f (TAIL xs) ys))
pub fn union() -> Term {
    app(
        Z(),
        abs!(
            3, // f, xs, ys
            app!(
                if_else(),
                app(is_nil(), Var(2)),
                Var(1),
                app!(
                    if_else(),
                    app(is_nil(), Var(1)),
                    Var(2),
                    app!(
                        if_else(),
                        app!(lt(), app(head(), Var(2)), app(head(), Var(1))),
                        app!(
                            cons(),
                            app(head(), Var(2)),
                            app!(Var(3), app(tail(), Var(2)), Var(1))
                        ),
                        app!(
                            if_else(),
                            app!(lt(), app(head(), Var(1)), app(head(), Var(2))),
                            app!(
                                cons(),
                                app(head(), Var(1)),
                                app!(Var(3), Var(2), app(tail(), Var(1)))
                            ),
                            app!(
                                cons(),
                                app(head(), Var(1)), // or Var(2); they’re equal
                                app!(Var(3), app(tail(), Var(2)), app(tail(), Var(1)))
                            )
                        )
                    )
                )
            )
        ),
    )
}

fn main() {
    // union tests
    let xs = vec![0.into_church(), 2.into_church()].into_pair_list();
    let ys = vec![1.into_church(), 3.into_church()].into_pair_list();
    let expected = vec![
        0.into_church(),
        1.into_church(),
        2.into_church(),
        3.into_church(),
    ]
    .into_pair_list();

    assert_eq!(beta(app!(union(), nil(), nil()), NOR, 0), nil());
    assert_eq!(
        beta(
            app!(union(), vec![0.into_church()].into_pair_list(), nil()),
            NOR,
            0
        ),
        vec![0.into_church()].into_pair_list()
    );
    assert_eq!(
        beta(app!(union(), xs.clone(), ys.clone()), NOR, 0),
        expected
    );

    let hard_xs = vec![0.into_church(), 2.into_church(), 3.into_church()].into_pair_list();
    let hard_ys = vec![1.into_church(), 3.into_church(), 5.into_church()].into_pair_list();

    // hard test
    assert_eq!(
        beta(app!(union(), hard_xs.clone(), hard_ys.clone()), NOR, 0),
        vec![
            0.into_church(),
            1.into_church(),
            3.into_church(),
            5.into_church()
        ]
        .into_pair_list()
    );
}
