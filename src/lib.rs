#![feature(tuple_indexing)]

#[cfg(test)]
extern crate hamcrest;

use std::collections::HashMap;

#[deriving(PartialEq,Show,Clone)]
pub struct Value(pub int);

#[deriving(PartialEq,Show,Clone)]
pub enum Exp {
    Constant(Value),
    Symbol(Name),
    Begin(Vec<Box<Exp>>),
    Set(Name, Box<Exp>),
}

#[deriving(PartialEq,Show,Clone,Hash,Eq)]
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
        self.map.insert(key, value).unwrap().0 == 0
    }

    pub fn find(&self, k: &Name) -> Option<&Value> {
        self.map.get(k)
    }

    pub fn locate(&mut self, _: Name) -> &mut Env {
        self // TODO: find outer envs
    }
}

pub fn eval(exp: Exp, env: &mut Env) -> Exp {
    match exp {
        Exp::Constant(val) => Exp::Constant(val),
        Exp::Symbol(name) => Exp::Constant(*(env.clone()).locate(name.clone()).find(&name).unwrap()),
        Exp::Begin(vec) => {
            let mut result = Exp::Constant(Value(0i)); // FIXME: probably should be 
                                                       // an iterator?
            for e in vec.iter() {
                result = eval(*e.clone(), env)
            }

            result
        },
        Exp::Set(name, exp) => {
            let result = match eval(*exp, env) {
                Exp::Constant(Value(name)) => Value(name),
                _ => panic!("Not a value, bub"),
            };

            env.locate(name.clone()).insert(name, result);

            Exp::Constant(result)
        },
    }
}

pub fn tokenize(program: String) -> Vec<String> {
    let mut program = program;

    program = std::str::replace(program.as_slice(), "(", " ( ");
    program = std::str::replace(program.as_slice(), ")", " ) ");

    program.as_slice()
        .split(' ')
        .filter(|s| *s != "") // we have to remove the extras
        .map(|s| s.to_string())
        .collect()
}

pub fn read_from(tokens: &mut Vec<String>) -> Exp {
    if tokens.len() == 0 {
        panic!("unexpected EOF while reading");
    }

    let token = tokens.pop().unwrap(); // we just checked from len() == 0

    if token.as_slice() == ")" {
        panic!("unexpected )");
    }
    
    let number: Option<int> = from_str(token.as_slice());

    match number {
        Some(num) => Exp::Constant(Value(num)),
        None => panic!("lol"),
    }
}

#[cfg(test)]
mod test {
    use hamcrest::{assert_that, equal_to, is};
    use Exp::{
        Constant,
        Symbol,
        Begin,
        Set,
    };
    use std::task;
    use super::eval;
    use super::tokenize;
    use super::read_from;
    use super::Name;
    use super::Value;
    use super::Env;

    #[test]
    fn test_eval_constant() {
        let exp = Constant(Value(5));

        assert_that(eval(exp.clone(), &mut Env::new()), is(equal_to(exp)));
    }

    #[test]
    fn test_env_like_hash_map() {
        let env = &mut Env::new();
        let key = Name("key".to_string());
        let value = Value(5i);

        //env.insert(key.clone(), value.clone());
        //assert_that(env.find(&key).unwrap(), is(equal_to(&value)));
    }

    #[test]
    fn test_eval_symbol() {
        let env = &mut Env::new();
        let name = Name("key".to_string());
        let value = Value(5i);
        //env.insert(name.clone(), value);
        
        let exp = Symbol(name);

        //assert_that(eval(exp.clone(), env), is(equal_to(Constant(value))));
    }

    #[test]
    fn test_eval_begin() {
        let constant = Constant(Value(5));
        let begin = Begin(vec![box constant.clone(), box constant.clone()]);

        assert_that(eval(begin, &mut Env::new()), is(equal_to(constant)));
    }

    #[test]
    fn test_eval_set() {
        let name = Name("Foo".to_string());
        let constant = Constant(Value(5));
        let set = box Set(name.clone(), box constant.clone());
        let symbol = box Symbol(name);

        let begin = Begin(vec![set, symbol]);

        //assert_that(eval(begin, &mut Env::new()), is(equal_to(constant)));
    }

    #[test]
    fn test_tokenize() {
        let program = "(set! twox (* x 2))".to_string();
        let tokenized = vec!["(".to_string(),
            "set!".to_string(),
            "twox".to_string(),
            "(".to_string(),
            "*".to_string(),
            "x".to_string(),
            "2".to_string(),
            ")".to_string(),
            ")".to_string()];

        assert_that(tokenize(program), is(equal_to(tokenized)));
    }

    #[test]
    fn test_read_from_zero() {
        //let res = task::try(proc() {
        //    read_from(&mut vec![]);
        //});

        //assert!(res.is_err());
    }

    #[test]
    fn test_read_from_lparen() {
        //let res = task::try(proc() {
        //    read_from(&mut vec![")".to_string()]);
        //});

        //assert!(res.is_err());
    }

    #[test]
    fn test_read_from_atom() {
        let atom = read_from(&mut vec!["5".to_string()]);

        assert_that(atom, is(equal_to(Constant(Value(5)))));
    }
}

