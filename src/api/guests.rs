//! # Guests
//!
//! Endpoints available for guests

use crate::api::endpoint::Endpoint;
use http::Method;
use std::borrow::Cow;

/// Retrieves a guest identified by their name.
#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Guest<'a> {
    #[doc = "`name` of the guest. The name is case-insensitive."]
    name: Cow<'a, str>,
}

impl<'a> Guest<'a> {
    /// Create a builder for this endpoint.
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
