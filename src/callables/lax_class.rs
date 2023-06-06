use std::{
    fmt, collections::HashMap,
    sync::{Arc, Mutex},
};

use lazy_static::__Deref;

use crate::{
    interpreter::Interpreter,
    token::Value,
    error::RuntimeError
};

use super::{
    lax_object::LaxObject,
    lax_functions::LaxFn,
    Callable,
    Call,
};

#[derive(Clone)]
pub struct LaxClass {
    pub name: String,
    pub methods: HashMap<String, LaxFn>,
}

impl LaxClass {
    pub fn find_method(&self, name: String) -> Option<Arc<Mutex<Value>>> {
        self.methods
            .get(&name)
            .cloned()
            .map(|method| 
                Arc::new(Mutex::new(Value::Callable(Callable::LaxFn(method))))
            )
    }
}

impl Call for LaxClass {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Arc<Mutex<Value>>>
        ) -> Result<Arc<Mutex<Value>>, RuntimeError> {
        let object = Arc::new(Mutex::new(LaxObject::new(self.clone())));
        let value = Arc::new(Mutex::new(Value::LaxObject(object)));

        if let Some(binding) = self.find_method("init".to_string()) {
            let method = binding.lock().unwrap();

            if let Value::Callable(Callable::LaxFn(initializer)) = method.deref() {
                initializer.bind(Arc::clone(&value)).call(interpreter, args)?;
            }
        }
        Ok(value)
    }

    fn arity(&self) -> usize {
        let value = match self.find_method("init".to_string()) {
            Some(initializer) => initializer,
            None => return 0,
        };
        let arity = match value.lock().unwrap().deref() {
            Value::Callable(func) => func.arity(),
            _ => 0,
        };
        arity
    }
}

impl fmt::Display for LaxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}", self.name) 
    }
}
