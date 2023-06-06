use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::fmt;

use crate::{
    interpreter::Interpreter,
    environment::Environment,
    error::RuntimeError,
    ast::stmt::Func,
    token::{
        Value,
        Token,
        TokenType,
    },
};

use super::Call;

#[derive(Clone)]
pub struct LaxFn {
    pub declaration: Func,
    pub closure: Arc<Mutex<Environment>>,
    is_init: bool,
}

impl LaxFn {
    pub fn new(declaration: Func, closure: Arc<Mutex<Environment>>,
        is_init: bool) -> Self {
        LaxFn { declaration , closure, is_init}
    }

    pub fn bind(&self, object: Arc<Mutex<Value>>) -> Self {
        let mut env = Environment::new(Some(Arc::clone(&self.closure)));
        env.define("this".to_string(), object);
        let env = Arc::new(Mutex::new(env));
        LaxFn::new(self.declaration.clone(), env, self.is_init)
    }
}

impl Call for LaxFn {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Arc<Mutex<Value>>>
        ) -> Result<Arc<Mutex<Value>>, RuntimeError> {
        let env = Environment::new_wrapped(Some(Arc::clone(&self.closure)));
        let params = &self.declaration.params;
        for (i, param) in params.iter().enumerate() {
            let arg = args.get(i).unwrap().clone();
            env.lock().unwrap().define(param.lexeme.clone(), arg);
        }

        let output = interpreter.execute_block(&self.declaration.body, env)?;

        if let Some(val) = output {
            return Ok(val)
        }
        if self.is_init {
            let sudo_token = Token::new(
                TokenType::Fn,
                "this".to_string(),
                Arc::new(Mutex::new(Value::None)),
                0
            );
            return self.closure.lock().unwrap().get_at(0, Rc::new(sudo_token))
        }

        Ok(Arc::new(Mutex::new(Value::None)))
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl fmt::Display for LaxFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.declaration)
    }
}
