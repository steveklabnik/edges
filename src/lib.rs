#[cfg(test)]
extern crate hamcrest;

use std::collections::HashMap;

#[deriving(PartialEq)]
#[deriving(Show)]
#[deriving(Clone)]
pub struct Value(pub int);

#[deriving(PartialEq)]
#[deriving(Show)]
#[deriving(Clone)]
pub enum Exp {
    Constant(Value),
    Symbol(Name),
}

#[deriving(PartialEq)]
#[deriving(Show)]
#[deriving(Clone)]
#[deriving(Hash)]
#[deriving(Eq)]
pub struct Name(pub String);

#[deriving(Clone)]
pub struct Env {
    map: HashMap<Name, Value>,
}

impl Env {
    pub fn new() -> Env {
        Env { map: HashMap::new() }
    }

    pub fn insert(&mut self, key: Name, value: Value) -> bool {
        self.map.insert(key, value)
    }

    pub fn find(&self, k: &Name) -> Option<&Value> {
        self.map.find(k)
    }

    pub fn locate(&self, k: Name) -> &Env {
        self
    }
}

pub fn eval(exp: Exp, env: Env) -> Exp {
    match exp {
        Constant(val) => Constant(val),
        Symbol(name) => Constant(*env.locate(name.clone()).find(&name).unwrap()),
    }
}

#[cfg(test)]
mod test {
    use hamcrest::{assert_that, equal_to, is};
    use {
        Constant,
        Value,
        eval,
        Env,
        Name,
        Symbol,
    };

    #[test]
    fn test_eval_constant() {
        let exp = Constant(Value(5));

        assert_that(eval(exp.clone(), Env::new()), is(equal_to(exp)));
    }

    #[test]
    fn test_env_like_hash_map() {
        let env = &mut Env::new();
        let key = Name("key".to_string());
        let value = Value(5i);

        env.insert(key.clone(), value.clone());
        assert_that(env.find(&key).unwrap(), is(equal_to(&value)));
    }

    #[test]
    fn test_eval_symbol() {
        let env = &mut Env::new();
        let name = Name("key".to_string());
        let value = Value(5i);
        env.insert(name.clone(), value);
        
        let exp = Symbol(name);

        assert_that(eval(exp.clone(), env.clone()), is(equal_to(Constant(value))));
    }
}

