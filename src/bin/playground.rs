use lambda_calculus::*;

fn main() {
    let true_s = format!("λx.λy.x");
    let false_s = format!("λx.λy.y");
    let incr_s = format!("λn.λf.λy.f ((n f) y)");
    // TODO: problem with needing to a-substitute

    // Boolean logic
    let true_l = parse(&format!("λx.λy.x"), Classic).unwrap();
    let false_l = parse(&format!("λx.λy.y"), Classic).unwrap();
    let and_l = parse(&format!("λa.λb.((a b) a)"), Classic).unwrap();
    let or_l = parse(&format!("λa.λb.((a a) b)"), Classic).unwrap();
    let not_l = parse(&format!("λb.((b ({false_s})) ({true_s}))"), Classic).unwrap();
    let if_l = parse(&format!("λp.λt.λf.(p t f)"), Classic).unwrap();

    // Arithmetic
    let succ_l = parse(&format!("λn.λf.λx.f (n f x)"), Classic).unwrap();
    let incr_l = parse(&format!("λn.λf.λy.f ((n f) y)"), Classic).unwrap();
    let plus_l = parse(&format!("λm.λn.(m {incr_s}) n"), Classic).unwrap();

    let mut expr = app!(
        if_l.clone(),
        true_l.clone(),                                                 // condition
        app!(plus_l.clone(), 2.into_church(), 1.into_church()).clone(), // true case
        false_l.clone()                                                 // false case
    );

    println!("{} order β-reduction steps for PRED 1 are:", NOR);

    println!("{}", expr);
    while expr.reduce(NOR, 1) != 0 {
        println!("{}", expr);
    }
}
