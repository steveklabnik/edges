extern crate edges;

use edges::{
    eval,
    Constant,
    Value,
};

fn main() {
    println!("{}", eval(Constant(Value(5))));
}
