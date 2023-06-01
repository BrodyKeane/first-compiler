use std::{fmt, collections::HashMap, rc::Rc};

use crate::{token::{Value, Token}, error::RuntimeError};

use super::lax_class::LaxClass;

#[derive(PartialEq, Clone)]
pub struct LaxObject {
    class: LaxClass,
    fields: HashMap<String, Rc<Value>>,
}

impl LaxObject {
    pub fn new(class: LaxClass) -> Self {
        LaxObject {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, token: Rc<Token>) -> Result<Rc<Value>, RuntimeError> {
        match self.fields.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                let message = format!("Undefined property '{}'", token.lexeme);
                Err(RuntimeError::new(token, &message))
            }
        }
    }

    pub fn set(&mut self, token: Rc<Token>, value: Rc<Value>) {
        self.fields.insert(token.lexeme.clone(), value);
    }
}

impl fmt::Display for LaxObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
