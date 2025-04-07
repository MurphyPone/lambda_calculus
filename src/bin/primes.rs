use blc::from_bits;
use lambda_calculus::combinators::Z;
use lambda_calculus::{
    data::{
        boolean::not,
        list::pair::{cons, filter, map, nil, take},
        num::church::mul,
    },
    *,
};

mod contains;
mod from;
mod minus;
mod multiples;
mod union;

pub fn primes() -> Term {
    app(
        Z(),
        abs!(1, {
            // Step 1: Generate multiples of `n` using map (λx. MUL n x) over the natural numbers
            let multiples = abs!(
                1,
                app!(
                    map(),
                    abs!(1, app!(mul(), Var(2), Var(1))), // n * x
                    from::from(Var(1))                    // Generate the list starting from n
                )
            );

            // Step 2: Union of all composites: union(map(multiples(prime)), primes)
            let composite_stream = app!(
                union::union(),
                app!(map(), multiples.clone(), Var(1)) // Apply map to multiples for the current prime
            );

            // Step 3: Natural numbers starting at 3 to check the next prime candidates
            let natural_numbers = from::from(3.into_church());

            // Step 4: Subtract the composite numbers from the natural numbers (to filter primes)
            let rest = app!(minus::minus(), natural_numbers, composite_stream);

            // Step 5: Prepend 2 to the resulting stream: CONS 2 rest
            app!(cons(), 2.into_church(), rest)
        }),
    )
}

fn main() {
    assert_eq!(
        beta(app!(take(), 0.into_church(), primes()), NOR, 0),
        vec![].into_pair_list(),
    );
    assert_eq!(
        beta(app!(take(), 1.into_church(), primes()), NOR, 0),
        vec![2.into_church(),].into_pair_list(),
    );
    assert_eq!(
        beta(app!(take(), 2.into_church(), primes()), NOR, 0),
        vec![2.into_church(), 3.into_church(),].into_pair_list(),
    );
    // TODO: here we can see it's not subtracting the composites
    assert_eq!(
        beta(app!(take(), 3.into_church(), primes()), NOR, 0),
        vec![2.into_church(), 3.into_church(), 4.into_church()].into_pair_list(),
    );
    assert_eq!(
        beta(app!(take(), 4.into_church(), primes()), NOR, 0),
        vec![
            2.into_church(),
            3.into_church(),
            4.into_church(),
            5.into_church(),
        ]
        .into_pair_list(),
    );
    assert_eq!(
        beta(app!(take(), 5.into_church(), primes()), NOR, 0),
        vec![
            2.into_church(),
            3.into_church(),
            4.into_church(),
            5.into_church(),
            6.into_church(),
        ]
        .into_pair_list(),
    );

    assert_eq!(
        beta(app!(take(), 3.into_church(), primes()), NOR, 0),
        vec![2.into_church(), 3.into_church(), 5.into_church(),].into_pair_list()
    );
}
