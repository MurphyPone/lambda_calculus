use lambda_calculus::{
    data::{
        boolean::{fls, not, tru},
        list::pair::{filter, head, is_nil},
        num::church::eq,
    },
    *,
};

// TODO: rename me `in()`
pub fn contains(x: Term, xs: Term) -> Term {
    let predicate = abs!(1, app!(eq(), x.clone(), Var(1)));
    let filtered = app!(filter(), predicate, xs.clone());
    app!(not(), app!(is_nil(), filtered))
}

fn main() {
    // [0, 2, 3]
    let list = || vec![0.into_church(), 2.into_church(), 3.into_church()].into_pair_list();

    assert_eq!(beta(contains(0.into_church(), list()), NOR, 0), tru());
    assert_eq!(beta(contains(1.into_church(), list()), NOR, 0), fls());
    assert_eq!(
        beta(contains(1.into_church(), vec![].into_pair_list()), NOR, 0),
        fls()
    );

    // Check if head of a list is in other list
    assert_eq!(
        beta(
            contains(app(head(), vec![0.into_church()].into_pair_list()), list()),
            NOR,
            0
        ),
        tru()
    );

    assert_eq!(
        beta(
            contains(app(head(), vec![5.into_church()].into_pair_list()), list()),
            NOR,
            0
        ),
        fls()
    );

    // let mut ex = contains(2.into_church(), list());

    // // println!("contains \t= {}", beta(contains, NOR, 0));
    // let mut t = 0;
    // while ex.reduce(NOR, 1) != 0 {
    //     println!();
    //     println!("{}: {}", t, ex);
    //     t += 1;
    // }
}
