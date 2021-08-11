use crate::api::variables::VariableId;

use super::Variable;

impl<'a> From<Variable<'a>> for VariableId<'a> {
    fn from(value: Variable<'a>) -> Self {
        value.id
    }
}
