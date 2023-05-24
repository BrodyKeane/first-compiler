use std::{fmt, rc::Rc};

use crate::{
    token::Value,
    interpreter::Interpreter,
    ast::stmt::Stmt,
};

pub trait Call {
    fn call(&self, interpreter: &Interpreter, args: Vec<Rc<Value>>) -> Value;
    fn arity(&self) -> usize;
}

pub enum Callable {
    NativeFn(NativeFn),
    //Func(Func),
}

impl Callable {
    pub fn new_native_fn(func: Box<dyn Fn(&Interpreter, Vec<Rc<Value>>) -> Value>, arity: usize) -> Self {
        Callable::NativeFn(NativeFn { func, arity })
    }

}

pub struct NativeFn {
    pub func: Box<dyn Fn(&Interpreter, Vec<Rc<Value>>) -> Value>,
    pub arity: usize,
}

impl Call for NativeFn {
    fn call(&self, interpreter: &Interpreter, args: Vec<Rc<Value>>) -> Value {
        (self.func)(interpreter, args)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

pub struct Func {
    declaration: Stmt
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

impl std::ops::Deref for Callable {
    type Target = dyn Call; // Define the associated Target type as the type you want to unwrap to

    fn deref(&self) -> &Self::Target {
        match self {
            Callable::NativeFn(func) => func,
            //Callable::Func(func) => func,
        }
    }
}
