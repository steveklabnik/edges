extern crate edges;

#[cfg(not(test))]
use edges::{
    eval,
    Constant,
    Value,
    Env,
};

#[cfg(not(test))]
fn main() {
    println!("{}", eval(Constant(Value(5)), &mut Env::new()));
}
