use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    rc::Rc,
    fmt,
};

use crate::{token::{Value, Token}, error::RuntimeError};

use super::{
    lax_class::LaxClass,
    Callable,
};

#[derive(Clone)]
pub struct LaxObject {
    class: LaxClass,
    fields: HashMap<String, Arc<RwLock<Value>>>,
}

impl LaxObject {
    pub fn new(class: LaxClass) -> Self {
        LaxObject {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, token: Rc<Token>, object: Arc<RwLock<Value>>
        ) -> Result<Arc<RwLock<Value>>, RuntimeError> {
        if let Some(field) = self.fields.get(token.lexeme.as_str()) {
            return Ok(Arc::clone(&field))
        }
        if let Some(binding) = self.class.find_method(token.lexeme.as_str()) {
            let value = binding.write().unwrap();
            if let Value::Callable(Callable::LaxFn(method)) = &*value {
                let binding = method.bind(object);
                let func = Value::Callable(Callable::LaxFn(binding));
                return Ok(Arc::new(RwLock::new(func)))
            }
        }
        let message = format!("Undefined property '{}'", token.lexeme);
        Err(RuntimeError::new(token, &message))
    }

    pub fn set(&mut self, token: Rc<Token>, value: Arc<RwLock<Value>>) {
        self.fields.insert(token.lexeme.to_string(), value);
    }
}

impl fmt::Display for LaxObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance", self.class)
    }
}

impl fmt::Debug for LaxObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance:", self.class)?;
        for (k, v) in &self.fields{
            writeln!(f, "{}: {}", k, v.read().unwrap())?;
        }
        Ok(())
    }
}
