#[cfg(test)]
extern crate hamcrest;

use std::fmt;

#[deriving(PartialEq)]
pub struct Value(pub int);

impl fmt::Show for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Value(val) = *self;

        write!(f, "{}", val)
    }
}

pub enum Atom {
    Constant(Value),
}

pub fn eval(atom: Atom) -> Value {
    match atom {
        Constant(val) => val,
    }
}

#[cfg(test)]
mod test {
    use hamcrest::{assert_that, equal_to, is};
    use {
        Constant,
        Value,
        eval,
    };

    #[test]
    fn test_eval_constant() {
        let val = Constant(Value(5));

        assert_that(eval(val), is(equal_to(Value(5))));
    }
}

