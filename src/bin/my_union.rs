use lambda_calculus::data::boolean::if_else;
use lambda_calculus::data::list::pair::{foldr, map, take};
use lambda_calculus::data::num::church::{eq, leq, lt, mul, succ};
use lambda_calculus::{
    combinators::Z,
    data::list::pair::{cons, head, is_nil, nil, tail},
    *,
};

pub fn merge() -> Term {
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

pub fn merge_no_dup() -> Term {
    app(
        Z(),
        abs!(3, {
            let recurse = Var(3);
            let xs = Var(2);
            let ys = Var(1);

            app!(
                is_nil(),
                xs.clone(),
                ys.clone(),
                app!(
                    is_nil(),
                    ys.clone(),
                    xs.clone(),
                    app!(
                        leq(),
                        app(head(), xs.clone()),
                        app(head(), ys.clone()),
                        app!(
                            if_else(),
                            app!(eq(), app(head(), xs.clone()), app(head(), ys.clone())),
                            // skip one copy if they’re equal
                            app!(recurse.clone(), app(tail(), xs.clone()), app(tail(), ys.clone())),
                            app!(
                                cons(),
                                app(head(), xs.clone()),
                                app!(recurse.clone(), app(tail(), xs.clone()), ys.clone())
                            )
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

pub fn my_union() -> Term {
    app!(
        foldr(),
        merge(),
        nil() // initial accumulator
    )
}

fn main() {

    let a = vec![0.into_church(), 2.into_church(), 4.into_church()].into_pair_list();
    let b = vec![1.into_church(), 3.into_church()].into_pair_list();
    let input = vec![a.clone(), b.clone()].into_pair_list();

    let expected = vec![
        0.into_church(),
        1.into_church(),
        2.into_church(),
        3.into_church(),
        4.into_church(),
    ].into_pair_list();

    assert_eq!(beta(app!(my_union(), input.clone()), NOR, 0), expected);

    // overlapping inputs

    let c = vec![0.into_church(), 3.into_church()].into_pair_list();
    let d = vec![1.into_church(), 3.into_church()].into_pair_list();
    let input2 = vec![c.clone(), d.clone()].into_pair_list();

    let expected = vec![
        0.into_church(),
        1.into_church(),
        3.into_church(),
        3.into_church(), // duplicate preserved
    ].into_pair_list();

    assert_eq!(beta(app!(my_union(), input2.clone()), NOR, 0), expected);



    // necessary helpers 
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

    // Step 1: Generate multiples of `n` using map (λx. MUL n x) over the natural numbers
    let multiples = abs({
        app!(
            map(),
            abs(app!(mul(), Var(2), Var(1))), // n * x
            from(Var(1))                    // Generate the list starting from n
        )
    });

    // the test case we care about

    // [2, 3, 5]
    let finite_primes = vec![2.into_church(), 3.into_church(), 5.into_church()].into_pair_list();
    
    // builds an (infinite) list of infinite lists: [[4, 6, 8, ...], [9, 12, 15, ...], [25, 30, 35, ...]]
    let list_of_multiples = app!(
        map(), 
        app!(take(), 2.into_church()), 
        app!(map(), multiples.clone(), finite_primes.clone()) 
        // after taking 2, should be left with: [[4, 6], [9, 12], [25, 30]]
    );

    assert_eq!(
        // outputs a 1D list: [4, 6, 8, 9, 10, 12]
        beta(app(my_union(), list_of_multiples.clone()), NOR, 0), 
        vec![4.into_church(), 6.into_church(), 
             9.into_church(), 12.into_church(), 
             25.into_church(), 30.into_church()
            ].into_pair_list()
    );

}
