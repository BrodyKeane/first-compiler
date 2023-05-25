use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

use crate::{
    interpreter::{Interpreter, RuntimeError},
    callables::{Callable, Call},
    environment::Environment,
    token::Value,
};

pub struct NativeDeclarations{
    globals: Arc<Mutex<Environment>>
}

pub struct NativeFn {
    pub func: Box<dyn Fn(&Interpreter, Vec<Rc<Value>>) -> Value>,
    pub arity: usize,
}

impl Call for NativeFn {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Rc<Value>>
        ) -> Result<Value, RuntimeError> {
        Ok((self.func)(interpreter, args))
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

impl NativeDeclarations {
    pub fn new(globals: Arc<Mutex<Environment>>) -> Self {
        NativeDeclarations { globals }
    }

    pub fn declare_natives(&mut self) -> Arc<Mutex<Environment>> {
        self.declare_clock();
        std::mem::replace(&mut self.globals, Environment::new_wrapped(None))
    }

    fn declare_clock(&mut self) {
        let clock_fn = |_: &Interpreter, _: Vec<Rc<Value>>| -> Value {
            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Failed to get current time")
                .as_secs_f64();
            Value::Num(current_time)
        };
        let value = Rc::new(Value::Callable(Callable::new_native_fn(Box::new(clock_fn), 0)));
        self.globals.lock().unwrap().define("clock".to_string(), value);
    }

}
