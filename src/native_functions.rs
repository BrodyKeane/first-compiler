use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::token::Value;
use crate::callable::Callable;

pub struct NativeDeclarations{
    globals: Arc<Mutex<Environment>>
}

impl NativeDeclarations {
    pub fn new(globals: Arc<Mutex<Environment>>) -> Self {
        NativeDeclarations { globals }
    }

    pub fn declare_natives(&mut self) -> Arc<Mutex<Environment>> {
        self.declare_clock();
        self.globals
    }

    fn declare_clock(&mut self) {
        let clock_fn = |interpreter: &Interpreter, _: Vec<Rc<Value>>| -> Value {
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
