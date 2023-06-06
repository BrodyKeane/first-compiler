use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
    fmt,
};

use crate::{
    interpreter::Interpreter,
    callables::{Callable, Call},
    environment::Environment,
    error::RuntimeError,
    token::Value,
};

pub struct NativeFn {
    pub func: Box<dyn Fn(&Interpreter, Vec<Arc<Mutex<Value>>>) -> Value>,
    pub arity: usize,
    pub name: String,
}

impl Call for NativeFn {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Arc<Mutex<Value>>>
        ) -> Result<Arc<Mutex<Value>>, RuntimeError> {
        let value = (self.func)(interpreter, args);
        Ok(Arc::new(Mutex::new(value)))
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

impl fmt::Display for NativeFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct NativeDeclarations {
    globals: Arc<Mutex<Environment>>
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
        let clock_fn = |_: &Interpreter, _: Vec<Arc<Mutex<Value>>>| -> Value {
            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Failed to get current time")
                .as_secs_f64();
            Value::Num(current_time)
        };
        let name = "clock".to_string();
        let callable = Callable::new_native_fn(name.clone(), Box::new(clock_fn), 0);
        let value = Arc::new(Mutex::new(Value::Callable(callable)));
        self.globals.lock().unwrap().define(name, value);
    }

}

