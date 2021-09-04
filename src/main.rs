mod tokenizer;
mod parser;
mod calculate;

use std::env::args;
use std::vec::Vec;

fn main() {
    let a: Vec<String> = args().skip(1).collect();
    let input = tokenizer::tokenize_string(&a[0]).unwrap();
    println!("{:?}", input);
    let n = parser::parse(&input).unwrap();
    println!("{:?}", n);
    let res = calculate::calculate(&n);
    println!("{:?}", res);
}
