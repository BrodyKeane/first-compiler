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

    pub fn define(&mut self, name: String, value: LitType) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: Token
        ) -> Result<&LitType, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(name) => Ok(name),
            None => {
                let message = format!("Undefined variable '{}'.", token.lexeme);
                Err(RuntimeError::new(token, &message))
            },
        }
    }
}


