use std::collections::{hash_map::Entry, HashMap};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::{
    token::{Value, Token},
    error::RuntimeError,
};

#[derive(Clone)]
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
                let message = format!("'{}' cannot be found in current scope.", token.lexeme);
                Err(RuntimeError::new(token, &message))
            }
        }
    }

    pub fn get_at(&self, distance: usize, token: Rc<Token>
        ) -> Result<Rc<Value> , RuntimeError> {
        let value = match self.ancestor(distance) {
            Some(env) => { 
                env.lock()
                   .unwrap()
                   .values
                   .get_mut(&token.lexeme)
                   .cloned()
            },
            None => { 
                let message = format!("The scope where '{}' is declared cannot be accessed", token.lexeme);
                return Err(RuntimeError::new(token, &message))
            }
        };
        match value {
            Some(val) => Ok(val),
            None => {
                let message = format!("'{}' cannot be found in eclosing scopes.", token.lexeme);
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
                let message = format!("Cannot assign a value to '{}' before it is declared", token.lexeme);
                Err(RuntimeError::new(token, &message))
            }
        }
    }

    pub fn assign_at(&mut self, distance: usize, token: Rc<Token>, value: Rc<Value>
        ) -> Result<(), RuntimeError> {
        match self.ancestor(distance) {
            Some(env) => { 
                env.lock()
                   .unwrap()
                   .values
                   .insert(token.lexeme.clone(), value);
            },
            None => { 
                let message = format!("Cannot assign value to '{}' in unreachable scope.", token.lexeme);
                return Err(RuntimeError::new(token, &message))
            }
        }
        Ok(())
    }

    fn ancestor(&self, distance: usize) -> Option<Arc<Mutex<Environment>>> {
        let mut env = Arc::new(Mutex::new(self.clone()));
        for _ in 0..distance {
            let enclosing = env.lock().unwrap().enclosing.clone();
            env = match enclosing {
                Some(env) => env,
                None => return None
            };
        }
        Some(env)
    }
}


