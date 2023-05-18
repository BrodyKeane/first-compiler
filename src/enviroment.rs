use std::collections::HashMap;

use crate::{
    token::{LitType, Token},
    interpreter::RuntimeError,
};

pub struct Enviroment {
    pub values: HashMap<String, LitType>
}

impl Enviroment {
    pub fn new() -> Self {
        Enviroment { values: HashMap::new() }
    }

    pub fn get(&self, token: &Token
        ) -> Result<&LitType, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(name) => Ok(name),
            None => {
                let message = format!("Undefined variable '{}'.", token.lexeme);
                Err(RuntimeError::new(token.clone(), &message))
            },
        }
    }

    pub fn define(&mut self, name: &String, value: LitType) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn assign(&mut self, token: Token, value: LitType
        ) -> Result<(), RuntimeError>{

        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme, value);
            return Ok(())
        }
        let message = format!("Undefined variable '{}'", token.lexeme);
        Err(RuntimeError::new(token, &message))
    }
}


