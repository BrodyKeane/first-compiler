use std::collections::{hash_map::Entry, HashMap};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::{
    token::{Value, Token},
    interpreter::RuntimeError,
};

#[derive(Debug)]
pub struct Environment {
    pub values: HashMap<String, Rc<Value>>,
    pub enclosing: Option<Arc<Mutex<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Arc<Mutex<Environment>>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn new_wrapped(enclosing: Option<Arc<Mutex<Environment>>>
        ) -> Arc<Mutex<Environment>> {
        Arc::new(Mutex::new(Environment::new(enclosing)))
    }
    
    pub fn define(&mut self, name: String, value: Rc<Value>) {
        self.values.insert(name, value);
    }

    //check innermost env for var else recursively check outer env 
    pub fn get(&self, token: Rc<Token>
        ) -> Result<Rc<Value> , RuntimeError> {
        if let Some(name) = self.values.get(&token.lexeme) {
            return Ok(name.clone())
        }

        match self.enclosing.clone() {
            Some(env) => env.lock().unwrap().get(token),
            None => {
                let message = format!("Undefined variable '{}'.", token.lexeme);
                Err(RuntimeError::new(token, &message))
            }
        }
    }

    //check innermost env for var else recursively check outer env 
    pub fn assign(&mut self, token: Rc<Token>, value: Rc<Value>
        ) -> Result<(), RuntimeError>{
        let entry = self.values.entry(token.lexeme.clone());

        if let Entry::Occupied(mut entry) = entry {
            *entry.get_mut() = value;
            return Ok(())
        }

        match self.enclosing.clone() {
            Some(env) => env.lock().unwrap().assign(token, value),
            None => {
                let message = format!("Undefined variable '{}'", token.lexeme);
                Err(RuntimeError::new(token, &message))
            }
        }
    }
}


