use lambda_calculus::{data::{list::pair::{cons, map, take}, num::church::mul}, *};

mod contains;
mod from;
mod minus;
mod multiples;
mod my_union;

fn main() {

    let multiples = abs({
        app!(
            map(),
            abs!(1, app!(mul(), Var(2), Var(1))), // n * x
            from::from(Var(1))                    // Generate the list starting from n
        )
    });

   // [2, 3, 5]
   let finite_primes = vec![2.into_church(), 3.into_church(), 5.into_church()].into_pair_list();
    
   // builds an (infinite) list of infinite lists: [[4, 6, 8, ...], [9, 12, 15, ...], [25, 30, 35, ...]]
   let finite_composite_stream = app!(
        my_union::my_union(),
        app!(
            map(),
            app(take(), 4.into_church()), 
            app!(map(), multiples.clone(), finite_primes.clone()) // Apply map to multiples for the current prime
        )
    );

    // Step 3: Generate the next prime candidates from the natural numbers
    let natural_numbers = app(
        app(take(), 9.into_church()), 
        from::from(3.into_church())
    );

    // Step 4: Subtract composites from natural numbers to get the primes
    let rest = app!(minus::minus(), natural_numbers.clone(), finite_composite_stream.clone());

    // Step 5: Return the result by appending the base case (2) with the remaining primes
    let mut result = app!(cons(), 2.into_church(), rest.clone());

    let mut t = 0;
    while result.reduce(NOR, 1) != 0 {
        // if t % 10000 == 0 {
        //     println!("β {t}");
        // }
        
        // println!();
        // println!("{}: {}", t, result);
        t += 1;
    }
    println!("\n\n{}: {}", t, result);
}
