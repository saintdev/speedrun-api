use std::borrow::Cow;

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Platforms {
    orderby: Option<PlatformsOrderBy>,
    direction: Option<Direction>,
}

impl Platforms {
    pub fn builder() -> PlatformsBuilder {
        PlatformsBuilder::default()
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformsOrderBy {
    Name,
    Released,
}

impl Endpoint for Platforms {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "/platforms".into()
    }

    fn query_parameters(&self) -> Result<std::borrow::Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Pageable for Platforms {}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Platform<'a> {
    id: Cow<'a, str>,
}

impl<'a> Platform<'a> {
    pub fn builder() -> PlatformBuilder<'a> {
        PlatformBuilder::default()
    }
}

impl Endpoint for Platform<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/platforms/{}", self.id).into()
    }
}
