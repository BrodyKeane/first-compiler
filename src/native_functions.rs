use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::token::Value;
use crate::callable::Callable;

pub fn declare_natives(globals: Arc<Mutex<Environment>>) -> Arc<Mutex<Environment>> {
    let clock_fn = |interpreter: &Interpreter, _: Vec<Rc<Value>>| -> Value {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get current time")
            .as_secs_f64();
        Value::Num(current_time)
    };
    let value = Rc::new(Value::Callable(Callable::new(Box::new(clock_fn), 0)));
    globals.lock().unwrap().define("clock".to_string(), value);

    globals
}
