use std::{
    sync::{Arc, Mutex, RwLock},
    collections::HashMap,
    rc::Rc,
    fmt,
};

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
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Arc<RwLock<Value>>>
        ) -> Result<Arc<RwLock<Value>>, RuntimeError>;

    fn arity(&self) -> usize;
}

#[derive(Debug)]
pub enum FuncType {
    Function, 
    Method,
    Initializer,
    None,
}

#[derive(Debug)]
pub enum Callable {
    NativeFn(NativeFn),
    LaxFn(LaxFn),
    LaxClass(LaxClass)
}

impl Callable {
    pub fn new_native_fn(name: String, func: Box<dyn Fn(&Interpreter, Vec<Arc<RwLock<Value>>>
        ) -> Value>, arity: usize) -> Self {
        Callable::NativeFn(NativeFn { name, func, arity })
    }

    pub fn new_lax_fn(declaration: Func, closure: Arc<Mutex<Environment>>,
        is_init: bool) -> Self {
        Callable::LaxFn(LaxFn::new(declaration, closure, is_init))
    }

    pub fn new_lax_class(name: Rc<String>, methods: HashMap<String, LaxFn>) -> Self {
        Callable::LaxClass(LaxClass { name, methods })
    }
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Callable::LaxFn(func) => write!(f, "{}", func),
            Callable::NativeFn(func) => write!(f, "{}", func),
            Callable::LaxClass(class) => write!(f, "{}", class),
        }
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
