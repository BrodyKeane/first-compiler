use std::{fmt, rc::Rc};

use crate::{
    token::Value,
    interpreter::{Interpreter, RuntimeError},
    ast::stmt::Func,
    environment::Environment,
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
}

pub struct NativeFn {
    pub func: Box<dyn Fn(&Interpreter, Vec<Rc<Value>>) -> Value>,
    pub arity: usize,
}

impl Call for NativeFn {
    fn call(&self, interpreter: &Interpreter, args: Vec<Rc<Value>>
        ) -> Result<Value, RuntimeError> {
        Ok((self.func)(interpreter, args))
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

pub struct LaxFn {
    declaration: Func,
    arity: usize,
}

impl Call for LaxFn {
    fn call(&self, interpreter: &Interpreter, args: Vec<Rc<Value>>
        ) -> Result<Value, RuntimeError> {
        let env = Environment::new_wrapped(Some(interpreter.globals));
        let params = self.declaration.params;
        for (i, param) in params.iter().enumerate() {
            let arg = args.get(i).unwrap().clone();
            env.lock().unwrap().define(param.lexeme, arg);
        }
        interpreter.execute_block(&self.declaration.body, env)?;
        Ok(Value::None)
    }

    fn arity(&self) -> usize {
        self.arity
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
