use lambda_calculus::combinators::{Y, Z};
use lambda_calculus::{
    data::{
        list::pair::{cons, map, take},
        num::church::mul,
    },
    *,
};

mod contains;
mod from;
mod minus;
mod multiples;
mod my_union;

// TODO: illustrate how the sieve is built
pub fn primes() -> Term {
    // The Z combinator for recursion
    app(
        Z(),
        abs!(1, {
            // Step 1: Define the multiples of n using the map function
            let multiples = abs(
                app!(
                    map(),
                    abs!(1, app!(mul(), Var(2), Var(1))), // n * x
                    from::from(Var(1))                    // Generate the list starting from n
                )
            );

            // Step 2: Generate composite numbers by union of multiples of known primes
            let composite_stream = app!(
                my_union::my_union(),
                map(), multiples.clone(), Var(1) // Apply map to multiples for the current prime
            );

            // Step 3: Generate the next prime candidates from the natural numbers
            let natural_numbers = from::from(3.into_church());

            // Step 4: Subtract composites from natural numbers to get the primes
            let rest = app!(minus::minus(), natural_numbers, composite_stream);

            // Step 5: Return the result by appending the base case (2) with the remaining primes
            app!(cons(), 2.into_church(), rest)
        }),
    )
}
 

fn main() {

    println!("{}", &format!("{:?}", primes()));

    assert_eq!(
        beta(app!(take(), 0.into_church(), primes()), APP, 0),
        vec![].into_pair_list(),
    );
    assert_eq!(
        beta(app!(take(), 1.into_church(), primes()), APP, 0),
        vec![2.into_church(),].into_pair_list(),
    );
    // assert_eq!(
    //     beta(app!(take(), 2.into_church(), primes()), NOR, 0),
    //     vec![2.into_church(), 3.into_church()].into_pair_list(),
    // );

    // // TODO: here we can see it's not subtracting the composites
    // assert_eq!(
    //     beta(app!(app(take(), 3.into_church()), primes()), NOR, 0),
    //     vec![2.into_church(), 3.into_church(), 5.into_church()].into_pair_list(),
    // );
}
