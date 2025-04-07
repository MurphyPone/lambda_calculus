use lambda_calculus::combinators::Y;
use lambda_calculus::data::boolean::{if_else, not, or};
use lambda_calculus::data::list::pair::{cons, filter, foldr, map, nil};
use lambda_calculus::data::num::church::{eq, lt, mul};
use lambda_calculus::*;

fn main() {
    // multiples = λn.λf.λx.f (mul n x)
    let multiples = abs!(
        3,
        app!(
            Var(2), //f
            app!(
                mul(),
                Var(1), // n
                Var(3)  // x
            )
        )
    );

    // contains = λl.λn.or (map (λx.eq n x) l)
    let contains = abs!(
        2,
        app!(
            or(),
            app!(
                map(),
                abs!(
                    1,
                    app!(eq(), Var(2), Var(3)) // (eq n x)
                ),
                Var(1)
            ) // l
        )
    );

    // minus = λxs.λys.filter (λx.not (contains ys x)) xs
    let minus = abs!(
        2,
        app!(
            filter(),
            abs!(
                1,
                app!(
                    not(),
                    app!(
                        contains, //contains
                        Var(2),   // ys
                        Var(3)    // xs
                    )
                )
            ),
            Var(1) // xs
        )
    );

    // insert_sorted = λx.λys.  // Vars 1, 2
    //   ys
    //     (λh.λt.              // Vars 3, 4
    //       if (lt x h)
    //         (cons x (cons h t))
    //         (if (eq x h)
    //           (cons h t)
    //           (cons h (insert_sorted x t))
    //         )
    //     )
    //     (cons x nil)

    let insert_sorted = app!(
        Y(),
        abs!(
            3,
            app!(
                Var(3), // ys
                abs!(
                    2, // λh.λt.
                    app!(
                        if_else(),
                        app!(lt(), Var(2), Var(4)), // (lt x h)
                        // (cons x (cons h t))
                        app!(cons(), Var(2), app!(cons(), Var(4), Var(5))),
                        app!(
                            if_else(),
                            app!(eq(), Var(2), Var(4)),   // (eq x h)
                            app!(cons(), Var(4), Var(5)), // then: (cons h t)
                            app!(cons(), Var(4), app!(Var(1), Var(2), Var(5))) // else: (cons h (insert_sorted x t))
                        )
                    )
                ),
                app!(cons(), Var(2), nil())
            )
        )
    );

    // merge = λx.λys.insert_sorted x ys
    let merge = abs!(2, app!(insert_sorted.clone(), Var(1), Var(2)));

    // union = λl1.λl2.foldr merge l2 l1
    let union_l = abs!(2, app!(foldr(), merge.clone(), Var(2), Var(1)));

    // primes = λf.cons two (minus (from three) (union (map multiples primes)))

    let mut applied = app!(
        insert_sorted.clone(),
        3.into_church(),
        vec![2, 4, 5].into_church()
    );

    let mut t = 0;
    while applied.reduce(NOR, 1) != 0 {
        println!("{}: {}", t, applied);
        println!();
        t += 1;
    }
    println!("{}", applied);

    // let mut expr = app!(
    //     contains.clone(),
    //     2.into_church(),
    //     vec![1, 2, 3].into_church()
    // );

    // println!("{} order β-reduction steps for PRED 1 are:", NOR);

    // println!("{}", expr);
    // while expr.reduce(NOR, 1) != 0 {
    //     println!("{}", expr);
    // }
}
