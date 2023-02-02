//! # Guests
//!
//! Endpoints available for guests

use crate::api::endpoint::Endpoint;

use std::borrow::Cow;

/// Retrieves a guest identified by their name.
#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct Guest<'a> {
    #[doc = "`name` of the guest. The name is case-insensitive."]
    name: Cow<'a, str>,
}

impl Guest<'_> {
    /// Create a builder for this endpoint.
    pub fn builder<'a>() -> GuestBuilder<'a> {
        GuestBuilder::default()
    }
}

impl Endpoint for Guest<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("/guests/{}", self.name).into()
    }
}
