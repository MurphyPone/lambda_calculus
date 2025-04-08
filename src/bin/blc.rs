use blc::from_bits;
use lambda_calculus::*;
use lambda_calculus::{
    app, beta, data::list::pair::take, parse, reduction::Order::NOR, term::Notation::Classic,
};

fn main() {
    let art = b"
    000100011001100101000110100
     000000101100000100100010101
     11110111          101001000
     11010000          111001101
     000000000010110111001110011
     11111011110000000011111001
     10111000
     00010110
    0000110110";

    let primes_blc_str = b"00010001100110010100011010000000010110000010010001010111110111101001000110100001110011010000000000101101110011100111111101111000000001111100110111000000101100000110110";

    let primes_blc = from_bits(primes_blc_str).unwrap();

    let primes_str = "λa.(λb.b (b ((λc.c c) (λc.λd.λe.e (λf.λg.g) ((λf.c c f ((λg.g g) (λg.f (g g)))) (λf.λg.λh.λi.i g (h (d f))))) (λc.λd.λe.b (e c))))) (λb.λc.c (λd.λe.d) b)";
    let primes_parsed = parse(primes_str, Classic).unwrap();

    // println!("unreduced:\t {}", primes_blc);
    // println!("reduced:\t {}", primes_parsed.clone());

    // TODO: his works weirdly
    // assert_eq!(
    //     beta(app!(take(), 3.into_church(), primes_parsed.clone()), NOR, 0),
    //     vec![2.into_church(), 3.into_church(), 5.into_church()].into_pair_list()
    // );

    // let first_1000_bits = b"000100010110011001010001101000000001011000001001000101011111011110100100011010000111001101000000000010110111001110011111110111100000000111110011011100101010000011100111001110011100111001110011100111001110011101000000111001110100000000001011011100111011110000000001000000101100000110110";
    // let to_lc = from_bits(first_1000_bits).unwrap();
    let mut to_lc_parse = parse("λ (λ 1 (1 ((λ 1 1) (λ λ λ 1 (λ λ 1)((λ 4 4 1 ((λ 1 1) (λ 2 (1 1))))(λ λ λ λ 1 3 (2 (6 4))))) (λ λ λ 4(1 3))))) (λ λ 1 (λ λ 2) 2)", DeBruijn).unwrap();


    let mut t = 0;
    while to_lc_parse.reduce(NOR, 1) != 0 {
        println!();
        println!("{}: {}", t, to_lc_parse);
        t += 1;
    }
}
