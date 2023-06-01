use std::{
    rc::Rc,
    fmt,
};

use crate::{
    callables::{Call, lax_object::LaxObject},
    interpreter::Interpreter,
    token::Value,
    error::RuntimeError
};

#[derive(Debug, PartialEq, Clone)]
pub struct LaxClass {
    pub name: String,
}

impl Call for LaxClass {
    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Rc<Value>>
        ) -> Result<Rc<Value>, RuntimeError> {
       Ok(Rc::new(Value::LaxObject(LaxObject::new(self.clone()))))
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
