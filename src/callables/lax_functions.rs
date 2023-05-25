use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::{
    interpreter::Interpreter,
    environment::Environment,
    error::RuntimeError,
    callables::Call,
    ast::stmt::Func,
    token::Value,
};

pub struct LaxFn {
    pub declaration: Func,
    pub closure: Arc<Mutex<Environment>>,
}

impl Call for LaxFn {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Rc<Value>>
        ) -> Result<Rc<Value>, RuntimeError> {
        let env = Environment::new_wrapped(Some(Arc::clone(&self.closure)));
        let params = &self.declaration.params;
        for (i, param) in params.iter().enumerate() {
            let arg = args.get(i).unwrap().clone();
            env.lock().unwrap().define(param.lexeme.clone(), arg);
        }
        let output = interpreter.execute_block(&self.declaration.body, env)?;
        match output {
            Some(val) => Ok(val),
            None => Ok(Rc::new(Value::None)),
        }
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
