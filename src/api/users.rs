use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, games::GameId, runs::RunEmbeds, Direction, Pageable};

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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct UserId<'a>(Cow<'a, str>);

impl<'a> UserId<'a> {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for UserId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for UserId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
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

#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct User<'a> {
    id: UserId<'a>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UserPersonalBests<'a> {
    #[serde(skip)]
    id: UserId<'a>,
    #[builder(default)]
    top: Option<i64>,
    #[builder(default)]
    series: Option<Cow<'a, str>>,
    #[builder(default)]
    game: Option<GameId<'a>>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<RunEmbeds>,
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

impl<'a> UserPersonalBestsBuilder<'a> {
    pub fn embed(&mut self, embed: RunEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = RunEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
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
