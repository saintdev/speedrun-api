use http::Method;
use serde::Serialize;
use std::borrow::Cow;

use super::{endpoint::Endpoint, Direction, Pageable};

/// Sorting options for users
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum UsersSorting {
    /// Sorts alphanumerically by the international name (default)
    #[serde(rename = "name.int")]
    NameInternational,
    /// Sorts alphanumerically by the Japanese name
    #[serde(rename = "name.jap")]
    NameJapanese,
    /// Sorts by the signup date
    Signup,
    /// Sorts by the user role
    Role,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Users<'a> {
    lookup: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    twitch: Option<Cow<'a, str>>,
    hitbox: Option<Cow<'a, str>>,
    twitter: Option<Cow<'a, str>>,
    speedrunslive: Option<Cow<'a, str>>,
    orderby: Option<UsersSorting>,
    direction: Option<Direction>,
}

#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct User<'a> {
    id: Cow<'a, str>,
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UserPersonalBests<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    top: Option<i64>,
    series: Option<Cow<'a, str>>,
    game: Option<Cow<'a, str>>,
}

impl<'a> Users<'a> {
    pub fn builder() -> UsersBuilder<'a> {
        UsersBuilder::default()
    }
}

impl<'a> User<'a> {
    pub fn builder() -> UserBuilder<'a> {
        UserBuilder::default()
    }
}

impl<'a> UserPersonalBests<'a> {
    pub fn builder() -> UserPersonalBestsBuilder<'a> {
        UserPersonalBestsBuilder::default()
    }
}

impl Default for UsersSorting {
    fn default() -> Self {
        Self::NameInternational
    }
}

impl Endpoint for Users<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/users".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for User<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/users/{}", self.id).into()
    }
}

impl Endpoint for UserPersonalBests<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/users/{}/personal-bests", self.id).into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Pageable for Users<'_> {}
