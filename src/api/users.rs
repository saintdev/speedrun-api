//! # Users
//!
//! Endpoints available for users
use std::{borrow::Cow, collections::BTreeSet, fmt::Display};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{endpoint::Endpoint, games::GameId, runs::RunEmbeds, Direction, Pageable};

/// Sorting options for users
#[derive(Default, Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum UsersSorting {
    /// Sorts alphanumerically by the international name (default)
    #[default]
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

/// Represents a user ID
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct UserId<'a>(Cow<'a, str>);

impl<'a> UserId<'a> {
    /// Create a new user ID
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

/// Retrieves a list of all users
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Users<'a> {
    #[doc = r"Performs a case-insensitive exact-match search for `lookup` across all user names, URLs and social profiles. Cannot be specified with any other filters."]
    lookup: Option<Cow<'a, str>>,
    #[doc = r"Only return users whose name/URL contain `name`. The comparison is case-insensitive."]
    name: Option<Cow<'a, str>>,
    #[doc = r"Search Twitch usernames"]
    twitch: Option<Cow<'a, str>>,
    #[doc = r"Search Hitbox usernames"]
    hitbox: Option<Cow<'a, str>>,
    #[doc = r"Search Twitter usernames"]
    twitter: Option<Cow<'a, str>>,
    #[doc = r"Search SpeedRunsLive usernames"]
    speedrunslive: Option<Cow<'a, str>>,
    #[doc = r"Sorting options for results."]
    orderby: Option<UsersSorting>,
    #[doc = r"Sort direction."]
    direction: Option<Direction>,
}

/// Retrieves a single user
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option))]
pub struct User<'a> {
    #[doc = r"User ID or username. Using an ID is recommended over a username, because usernames can change."]
    id: UserId<'a>,
}

/// Retrieves a list of runs representing the personal bests for a user
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UserPersonalBests<'a> {
    #[doc = r"User ID or username. Using an ID is recommended over a username, because usernames can change."]
    #[serde(skip)]
    id: UserId<'a>,
    #[doc = r"Only return PBs with a place equal or better than `top` (e.g. a value of `1` will return all world records for the given user)"]
    #[builder(default)]
    top: Option<i64>,
    #[doc = r"Series ID or abbreviation. When given, restricts results to that games and romhacks in that series."]
    #[builder(default)]
    series: Option<Cow<'a, str>>,
    #[doc = r"Game ID or abbreviation. When given, restricts results to that game"]
    #[builder(default)]
    game: Option<GameId<'a>>,
    #[builder(setter(name = "_embed"), private, default)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<RunEmbeds>,
}

impl<'a> Users<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> UsersBuilder<'a> {
        UsersBuilder::default()
    }
}

impl<'a> User<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> UserBuilder<'a> {
        UserBuilder::default()
    }
}

impl<'a> UserPersonalBests<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> UserPersonalBestsBuilder<'a> {
        UserPersonalBestsBuilder::default()
    }
}

impl<'a> UserPersonalBestsBuilder<'a> {
    /// Add an embedded resource to this result.
    pub fn embed(&mut self, embed: RunEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result.
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = RunEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
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
