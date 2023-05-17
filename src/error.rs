use std::error::Error;

pub struct ErrorStatus {
    pub had_compile_error: bool,
    pub had_runtime_error: bool,
}

impl ErrorStatus {
    pub fn new() -> Self {
        ErrorStatus { 
            had_compile_error: false,
            had_runtime_error: false,
        }
    }

    pub fn report_compile_error<E: Error>(&mut self, error: E) {
        eprintln!("{}", error);
        self.had_compile_error = true;
    }

    pub fn report_runtime_error<E: Error>(&mut self, error: E) {
        eprintln!("{}", error);
        self.had_runtime_error = true;
    }
}
