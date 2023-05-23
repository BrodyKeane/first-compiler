use std::collections::{hash_map::Entry, HashMap};

use crate::{
    token::{LitType, Token},
    interpreter::RuntimeError,
};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, LitType>,
    enclosing: Box<Option<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Box::new(enclosing),
        }
    }

    pub fn define(&mut self, name: &String, value: LitType) {
        self.values.insert(name.to_owned(), value);
    }

    //check innermost env for var else recursively check outer env 
    pub fn get(&self, token: &Token
        ) -> Result<&LitType, RuntimeError> {
        if let Some(name) = self.values.get(&token.lexeme) {
            return Ok(name)
        }

        match &*self.enclosing {
            Some(env) => env.get(token),
            None => {
                let message = format!("Undefined variable '{}'.", token.lexeme);
                Err(RuntimeError::new(token.clone(), &message))
            }
        }
    }

    //check innermost env for var else recursively check outer env 
    pub fn assign(&mut self, token: Token, value: LitType
        ) -> Result<(), RuntimeError>{
        let entry = self.values.entry(token.lexeme.clone());

        if let Entry::Occupied(mut entry) = entry {
            *entry.get_mut() = value;
            return Ok(())
        }

        match &mut *self.enclosing {
            Some(env) => env.assign(token, value),
            None => {
                let message = format!("Undefined variable '{}'", token.lexeme);
                Err(RuntimeError::new(token, &message))
            }
        }
    }
}


