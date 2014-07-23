extern crate edges;

use edges::{
    eval,
    Constant,
    Value,
    Env,
};

fn main() {
    println!("{}", eval(Constant(Value(5)), &mut Env::new()));
}
