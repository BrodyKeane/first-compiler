use std::collections::{hash_map::Entry, HashMap};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::fmt;

use crate::{
    token::{Value, Token},
    error::RuntimeError,
};

#[derive(Clone)]
pub struct Environment {
    pub values: HashMap<String, Arc<RwLock<Value>>>,
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
    
    pub fn define(&mut self, name: String, value: Arc<RwLock<Value>>) {
        self.values.insert(name, value);
    }

    //check innermost env for var else recursively check outer env 
    pub fn get(&self, token: Rc<Token>
        ) -> Result<Arc<RwLock<Value>> , RuntimeError> {
        if let Some(value) = self.values.get(token.lexeme.as_str()) {
            return Ok(Arc::clone(value))
        }

        match &self.enclosing {
            Some(env) => env.lock().unwrap().get(token),
            None => {
                let message = format!("'{}' cannot be found in current scope.", token.lexeme);
                Err(RuntimeError::new(token, &message))
            }
        }
    }

    pub fn get_at(&self, distance: usize, token: Rc<Token>
        ) -> Result<Arc<RwLock<Value>> , RuntimeError> {
        let value = match self.ancestor(distance) {
            Some(env) => { 
                env.lock()
                   .unwrap()
                   .values
                   .get_mut(token.lexeme.as_str())
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
    pub fn assign(&mut self, token: Rc<Token>, value: Arc<RwLock<Value>>
        ) -> Result<(), RuntimeError>{
        let entry = self.values.entry(token.lexeme.to_string());

        if let Entry::Occupied(mut entry) = entry {
            *entry.get_mut() = value;
            return Ok(())
        }

        match &self.enclosing {
            Some(env) => env.lock().unwrap().assign(token, value),
            None => {
                let message = format!("Cannot assign a value to '{}' before it is declared", token.lexeme);
                Err(RuntimeError::new(token, &message))
            }
        }
    }

    pub fn assign_at(&mut self, distance: usize, token: Rc<Token>, value: Arc<RwLock<Value>>
        ) -> Result<(), RuntimeError> {
        match self.ancestor(distance) {
            Some(env) => { 
                env.lock()
                   .unwrap()
                   .values
                   .insert(token.lexeme.to_string(), value);
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


impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Environment:")?;
        for (k, v) in &self.values {
            writeln!(f, "{}: {}", k, v.read().unwrap())?;
        }
        loop {
            let enclosing = match &self.enclosing {
                Some(env) => env,
                None => break,
            };

            for (k, v) in &enclosing.lock().unwrap().values {
                writeln!(f, "{}: {}", k, v.read().unwrap())?;
            }
        }
        Ok(())
    }
}
