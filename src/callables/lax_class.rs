use std::{
    fmt, collections::HashMap,
    sync::{Arc, Mutex, RwLock},
    rc::Rc,
};


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
    pub name: Rc<String>,
    pub methods: HashMap<String, LaxFn>,
    pub superclass: Option<Arc<RwLock<Value>>>,
}

impl LaxClass {
    pub fn find_method(&self, name: &str) -> Option<Arc<RwLock<Value>>> {
        let method = self.methods
            .get(name)
            .cloned()
            .map(|method| 
                Arc::new(RwLock::new(Value::Callable(Callable::LaxFn(method))))
            );
        if method.is_some() {return method}

        if let Some(value) = &self.superclass {
            if let Value::Callable(Callable::LaxClass(superclass)) = &*value.read().unwrap() {
                return superclass.find_method(name)
            }
        }
        method
    }
}

impl Call for LaxClass {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Arc<RwLock<Value>>>
        ) -> Result<Arc<RwLock<Value>>, RuntimeError> {
        let object = Arc::new(Mutex::new(LaxObject::new(self.clone())));
        let value = Arc::new(RwLock::new(Value::LaxObject(object)));

        if let Some(binding) = self.find_method("init") {
            let method = binding.write().unwrap();

            if let Value::Callable(Callable::LaxFn(initializer)) = &*method {
                initializer.bind(Arc::clone(&value)).call(interpreter, args)?;
            }
        }
        Ok(value)
    }

    fn arity(&self) -> usize {
        let value = match self.find_method("init") {
            Some(initializer) => initializer,
            None => return 0,
        };
        let arity = match &*value.read().unwrap() {
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

impl fmt::Debug for LaxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;
        for (k, _) in &self.methods {
            writeln!(f, "{}", k)?;
        }
        Ok(())
    }
}
