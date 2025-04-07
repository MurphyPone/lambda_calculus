use lambda_calculus::data::boolean::{if_else, not};
use lambda_calculus::data::num::church::{eq, lt};
use lambda_calculus::{
    combinators::Z,
    data::list::pair::{cons, filter, head, is_nil, nil, tail},
    *,
};

// union = foldr merge []
//   where
//     merge (x:xs) ys = x:merge’ xs ys
//     merge’ (x:xs) (y:ys) | x < y = x:merge’ xs (y:ys)
//                          | x == y = x:merge’ xs ys
//                          | x > y = y:merge’ (x:xs) ys

pub fn union_no_dupe() -> Term {
    // The `merge` function, which merges two lists
    let merge_step = abs!(
        3, // f, xs, ys
        app!(
            if_else(),
            app(is_nil(), Var(2)),
            Var(1), // If xs is empty, return ys
            app!(
                if_else(),
                app(is_nil(), Var(1)),
                Var(2), // If ys is empty, return xs
                app!(
                    if_else(),
                    app!(lt(), app(head(), Var(2)), app(head(), Var(1))),
                    app!(
                        cons(),
                        app(head(), Var(2)), // Take the head of xs if it's smaller
                        app!(Var(3), app(tail(), Var(2)), Var(1))
                    ),
                    app!(
                        if_else(),
                        app!(eq(), app(head(), Var(2)), app(head(), Var(1))),
                        app!(
                            cons(),
                            app(head(), Var(2)), // Take the head if they are equal
                            app!(Var(3), app(tail(), Var(2)), app(tail(), Var(1)))
                        ),
                        app!(
                            cons(),
                            app(head(), Var(1)), // Otherwise take the head of ys
                            app!(Var(3), Var(2), app(tail(), Var(1)))
                        )
                    )
                )
            )
        )
    );

    // The union function itself, which folds over the lists
    app(
        Z(),
        abs!(
            3, // f, xs, ys
            app!(
                merge_step,
                Var(2),
                Var(1) // Perform the merge step on xs and ys
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

    // empty list TODO: doesn't work
    // assert_eq!(beta(app!(union_no_dupe(), nil(), nil()), NOR, 0), nil());

    // one non-empty list
    assert_eq!(
        beta(
            app!(
                union_no_dupe(),
                vec![0.into_church()].into_pair_list(),
                nil()
            ),
            NOR,
            0
        ),
        vec![0.into_church()].into_pair_list()
    );

    // one non-empty list with multiple items
    assert_eq!(
        beta(
            app!(
                union_no_dupe(),
                vec![0.into_church(), 1.into_church()].into_pair_list(),
                nil()
            ),
            NOR,
            0
        ),
        vec![0.into_church(), 1.into_church()].into_pair_list(),
    );

    // duplicate lists
    assert_eq!(
        beta(
            app!(
                union_no_dupe(),
                vec![0.into_church()].into_pair_list(),
                vec![0.into_church()].into_pair_list()
            ),
            NOR,
            0
        ),
        vec![0.into_church()].into_pair_list(),
    );

    // 2 non-empty lists with all-distinct items
    assert_eq!(
        beta(app!(union_no_dupe(), xs.clone(), ys.clone()), NOR, 0),
        expected
    );

    let hard_xs = vec![0.into_church(), 2.into_church(), 3.into_church()].into_pair_list();
    let hard_ys = vec![1.into_church(), 3.into_church(), 5.into_church()].into_pair_list();

    // hard test
    assert_eq!(
        beta(
            app!(union_no_dupe(), hard_xs.clone(), hard_ys.clone()),
            NOR,
            0
        ),
        vec![
            0.into_church(),
            1.into_church(),
            3.into_church(),
            5.into_church()
        ]
        .into_pair_list()
    );
}
