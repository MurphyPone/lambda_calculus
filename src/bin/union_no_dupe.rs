use lambda_calculus::data::boolean::if_else;
use lambda_calculus::data::list::pair::{nil, cons, head, tail, is_nil};
use lambda_calculus::data::num::church::{lt, eq};
use lambda_calculus::combinators::Z;
use lambda_calculus::*;

// union = foldr merge []
//   where
//     merge (x:xs.clone()) ys = x:merge’ xs ys
//     merge’ (x:xs.clone()) (y:ys.clone()) | x < y = x:merge’ xs (y:ys.clone())
//                          | x == y = x:merge’ xs ys
//                          | x > y = y:merge’ (x:xs.clone()) ys

pub fn union_no_dupe() -> Term {

    // Define the core comparison and recursion logic of merge'
    let merge_prime = abs!(3, {
        let s = Var(3); // self-reference
        let xs = Var(2); // first list
        let ys = Var(1); // second list

        // If first list is empty, return second list
        app!(
            if_else(), 
            app!(is_nil(), xs.clone()), 
            ys.clone(),
            // If second list is empty, return first list
            app!(
                if_else(),
                app!(is_nil(), ys.clone()),
                xs.clone(),
                // Compare heads and choose appropriate branch
                app!(
                    if_else(),
                    app!(lt(), app!(head(), xs.clone()), app!(head(), ys.clone())),
                    // If x < y: cons x (merge' xs ys)
                    app!(cons(), app!(head(), xs.clone()), app!(s.clone(), app!(tail(), xs.clone()), ys.clone())),
                    app!(
                        if_else(),
                        app!(eq(), app!(head(), xs.clone()), app!(head(), ys.clone())),
                        // If x == y: cons x (merge' xs (tail ys))
                        app!(cons(), app!(head(), xs.clone()), app!(s.clone(), app!(tail(), xs.clone()), app!(tail(), ys.clone()))),
                        // If x > y: cons y (merge' xs (tail ys))
                        app!(cons(), app!(head(), ys.clone()), app!(s.clone(), xs.clone(), app!(tail(), ys.clone())))
                    )
                )
            )
        )
    });

    // Apply Z combinator to make merge_prime recursive
    let merge_prime_rec = app!(Z(), merge_prime.clone());

    // Define merge function that uses merge_prime_rec
    let merge = abs!(2, {
        let xs = Var(2); // first list
        let ys = Var(1); // second list

        // If both lists are empty, return nil
        app!(
            if_else(),
            app!(is_nil(), xs.clone()),
            app!(
                if_else(),
                app!(is_nil(), ys.clone()),
                nil(),
                ys.clone()
            ),
            // If first list is empty but second is not, return second list
            app!(
                if_else(),
                app!(is_nil(), ys.clone()),
                xs.clone(),
                // Otherwise, handle duplicates and merge
                app!(
                    if_else(),
                    app!(eq(), app!(head(), xs.clone()), app!(head(), ys.clone())),
                    // If x == y: cons x (merge' xs (tail ys))
                    app!(cons(), app!(head(), xs.clone()), app!(merge_prime_rec.clone(), app!(tail(), xs.clone()), app!(tail(), ys.clone()))),
                    app!(
                        if_else(),
                        app!(lt(), app!(head(), xs.clone()), app!(head(), ys.clone())),
                        // If x < y: cons x (merge' xs ys)
                        app!(cons(), app!(head(), xs.clone()), app!(merge_prime_rec.clone(), app!(tail(), xs.clone()), ys.clone())),
                        // If x > y: cons y (merge' xs (tail ys))
                        app!(cons(), app!(head(), ys.clone()), app!(merge_prime_rec.clone(), xs.clone(), app!(tail(), ys.clone())))
                    )
                )
            )
        )
    });

    // Return the merger function directly
    merge
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
    assert_eq!(beta(app!(union_no_dupe(), nil(), nil()), NOR, 0), nil());

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
            2.into_church(),
            3.into_church(),
            5.into_church()
        ]
        .into_pair_list()
    );
}
