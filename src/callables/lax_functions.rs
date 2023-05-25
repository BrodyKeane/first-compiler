use std::rc::Rc;

use crate::{
    interpreter::{Interpreter, RuntimeError},
    callables::Call,
    environment::Environment,
    ast::stmt::Func,
    token::Value,
};

pub struct LaxFn {
    declaration: Func,
}

impl Call for LaxFn {
    fn call(&self, interpreter: &Interpreter, args: Vec<Rc<Value>>
        ) -> Result<Value, RuntimeError> {
        let env = Environment::new_wrapped(Some(interpreter.globals));
        let params = self.declaration.params;
        for (i, param) in params.iter().enumerate() {
            let arg = args.get(i).unwrap().clone();
            env.lock().unwrap().define(param.lexeme, arg);
        }
        interpreter.execute_block(&self.declaration.body, env)?;
        Ok(Value::None)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
