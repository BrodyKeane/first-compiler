use std::{fmt, rc::Rc};

use crate::{
    token::Value,
    interpreter::Interpreter,
};

pub struct Callable {
    pub func: Box<dyn Fn(&Interpreter, Vec<Rc<Value>>) -> Value>,
    pub arity: usize,
}

impl Callable {
    pub fn new(func: Box<dyn Fn(&Interpreter, Vec<Rc<Value>>) -> Value>, arity: usize) -> Self {
        Callable { func, arity }
    }

    pub fn call(&self, interpreter: &Interpreter, args: Vec<Rc<Value>>) -> Value {
        (self.func)(interpreter, args)
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        // Compare the wrapped function by comparing their addresses
        std::ptr::eq(&*self, &*other)
    }
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn>")
    }
}
