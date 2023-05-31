use std::{
    rc::Rc,
    fmt,
};

use crate::{
    callables::{Call, lax_instance::LaxInstance},
    interpreter::Interpreter,
    token::Value,
    error::RuntimeError
};

#[derive(Debug, PartialEq, Clone)]
pub struct LaxClass {
    pub name: String,
}

impl Call for LaxClass {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Rc<Value>>
        ) -> Result<Rc<Value>, RuntimeError> {
       Ok(Rc::new(Value::LaxInstance(LaxInstance::new(self.clone()))))
    }

    fn arity(&self) -> usize {
       0 
    }
}

impl fmt::Display for LaxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", self.name) 
    }
}
