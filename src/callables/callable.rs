use std::{fmt, rc::Rc};

use crate::{
    token::Value,
    interpreter::{Interpreter, RuntimeError},
    ast::stmt::Func,
    callables::{
        native_functions::NativeFn,
        lax_functions::LaxFn,
    },
};
 
pub trait Call {
    fn call(&self, interpreter: &Interpreter, args: Vec<Rc<Value>>
        ) -> Result<Value, RuntimeError>;
    fn arity(&self) -> usize;
}

pub enum Callable {
    NativeFn(NativeFn),
    LaxFn(LaxFn),
}

impl Callable {
    pub fn new_native_fn(func: Box<dyn Fn(&Interpreter, Vec<Rc<Value>>) -> Value>, arity: usize) -> Self {
        Callable::NativeFn(NativeFn { func, arity })
    }

    pub fn new_lax_fn(declaration: Func) -> Self {
        Callable::LaxFn(LaxFn { declaration })
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

impl std::ops::Deref for Callable {
    type Target = dyn Call;

    fn deref(&self) -> &Self::Target {
        match self {
            Callable::NativeFn(func) => func,
            Callable::LaxFn(func) => func,
        }
    }
}
