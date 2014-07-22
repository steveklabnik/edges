extern crate edges;

#[cfg(test)]
extern crate hamcrest;

use edges::{
    eval,
    Constant,
    Value,
};

fn main() {
    println!("{}", eval(Constant(Value(5))));
}
