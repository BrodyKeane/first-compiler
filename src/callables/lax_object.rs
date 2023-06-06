use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    rc::Rc,
    fmt,
};

use lazy_static::__Deref;

use crate::{token::{Value, Token}, error::RuntimeError};

use super::{
    lax_class::LaxClass,
    Callable,
};

#[derive(Clone)]
pub struct LaxObject {
    class: LaxClass,
    fields: HashMap<String, Arc<Mutex<Value>>>,
}

impl LaxObject {
    pub fn new(class: LaxClass) -> Self {
        LaxObject {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, token: Rc<Token>, object: Arc<Mutex<Value>>
        ) -> Result<Arc<Mutex<Value>>, RuntimeError> {
        if let Some(field) = self.fields.get(&token.lexeme) {
            return Ok(field.clone())
        }
        if let Some(binding) = self.class.find_method(token.lexeme.clone()) {
            let value = binding.lock().unwrap();
            if let Value::Callable(Callable::LaxFn(method)) = value.deref() {
                let binding = method.bind(object);
                let func = Value::Callable(Callable::LaxFn(binding));
                return Ok(Arc::new(Mutex::new(func)))
            }
        }
        let message = format!("Undefined property '{}'", token.lexeme);
        Err(RuntimeError::new(token, &message))
    }

    pub fn set(&mut self, token: Rc<Token>, value: Arc<Mutex<Value>>) {
        self.fields.insert(token.lexeme.clone(), value);
    }
}

impl fmt::Display for LaxObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
