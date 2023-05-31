use std::{fmt, rc::Rc};
use std::sync::{Arc, Mutex};

use crate::{
    interpreter::Interpreter,
    environment::Environment,
    error::RuntimeError,
    ast::stmt::Func,
    token::Value,
    callables::{
        native_functions::NativeFn,
        lax_functions::LaxFn,
        lax_class::LaxClass,
    }
};
 
pub trait Call {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Rc<Value>>
        ) -> Result<Rc<Value>, RuntimeError>;
    fn arity(&self) -> usize;
}

pub enum Callable {
    NativeFn(NativeFn),
    LaxFn(LaxFn),
    LaxClass(LaxClass)
}

impl Callable {
    pub fn new_native_fn(func: Box<dyn Fn(&Interpreter, Vec<Rc<Value>>) -> Value>, arity: usize) -> Self {
        Callable::NativeFn(NativeFn { func, arity })
    }

    pub fn new_lax_fn(declaration: Func, closure: Arc<Mutex<Environment>>) -> Self {
        Callable::LaxFn(LaxFn { declaration, closure })
    }

    pub fn new_lax_class(name: String) -> Self {
        Callable::LaxClass(LaxClass { name })
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        // Compare the wrapped function by comparing their addresses
        std::ptr::eq(&self, &other)
    }
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "{:?}", self)
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
            Callable::LaxClass(class) => class,
        }
    }
}
