use std::fmt;

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
  use hamcrest::{assert_that, equal_to, contains};
}

