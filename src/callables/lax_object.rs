use std::{fmt, collections::HashMap, rc::Rc};

use crate::{token::{Value, Token}, error::RuntimeError};

use super::lax_class::LaxClass;

#[derive(PartialEq)]
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
}

impl fmt::Display for LaxObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
