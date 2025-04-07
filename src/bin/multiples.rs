use lambda_calculus::{
    combinators::Z,
    data::{
        list::pair::{cons, map, take},
        num::church::{mul, succ},
    },
    *,
};

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

pub fn from(n: Term) -> Term {
    iterate(succ(), n)
}

pub fn multiples(x: Term) -> Term {
    // Generate the list of natural numbers starting from 1
    let nat_seq = from(1.into_church()); // Generate [1, 2, 3, 4, 5, ...]

    // Map the multiplication function (n * x) over the list
    app!(
        map(),
        abs(app!(mul(), x.clone(), Var(1))), // Function: n * x
        nat_seq                              // Apply the function to the list of natural numbers
    )
}

fn main() {
    assert_eq!(
        beta(
            app!(take(), 2.into_church(), multiples(2.into_church())),
            NOR,
            0
        ),
        vec![2.into_church(), 4.into_church(),].into_pair_list()
    );

    // let mut ex = insert_sorted(1.into_church(), list());

    // let mut t = 0;
    // while ex.reduce(NOR, 1) != 0 {
    //     println!();
    //     println!("{}: {}", t, ex);
    //     t += 1;
    // }
}
