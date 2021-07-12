use http::Method;

use super::endpoint::Endpoint;

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Profile {}

impl Profile {
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::default()
    }
}

impl Endpoint for Profile {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "/profile".into()
    }

    fn requires_authentication(&self) -> bool {
        true
    }
}
