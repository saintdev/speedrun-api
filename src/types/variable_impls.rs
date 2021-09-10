use std::fmt::Display;

use crate::api::variables::VariableId;

use super::{Value, Variable};

impl<'a> From<Variable<'a>> for VariableId<'a> {
    fn from(value: Variable<'a>) -> Self {
        value.id
    }
}

impl Display for Variable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.label)
    }
}
