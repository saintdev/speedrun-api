use crate::api::endpoint::Endpoint;
use http::Method;
use std::borrow::Cow;

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Guest<'a> {
    name: Cow<'a, str>,
}

impl<'a> Guest<'a> {
    pub fn builder() -> GuestBuilder<'a> {
        GuestBuilder::default()
    }
}

impl Endpoint for Guest<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/guests/{}", self.name).into()
    }
}
