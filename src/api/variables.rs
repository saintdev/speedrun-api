use std::borrow::Cow;

use http::Method;

use super::endpoint::Endpoint;

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Variable<'a> {
    id: Cow<'a, str>,
}

impl<'a> Variable<'a> {
    pub fn builder() -> VariableBuilder<'a> {
        VariableBuilder::default()
    }
}

impl Endpoint for Variable<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/variables/{}", self.id).into()
    }
}
