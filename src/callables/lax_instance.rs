use std::fmt;

use super::lax_class::LaxClass;

#[derive(Debug, PartialEq)]
pub struct LaxInstance {
    class: LaxClass
}

impl LaxInstance {
    pub fn new(class: LaxClass) -> Self {
        LaxInstance{ class }
    }
}

impl fmt::Display for LaxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.class)
    }
}
