//! # Runs
//!
//! Endpoints available for runs.
use std::{
    borrow::Cow,
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

use http::Method;
use serde::{Deserialize, Serialize};

use super::{
    categories::CategoryId,
    endpoint::Endpoint,
    games::GameId,
    levels::LevelId,
    platforms::PlatformId,
    regions::RegionId,
    users::UserId,
    variables::{ValueId, VariableId},
    Direction, Pageable,
};

/// Embeds available for runs.
///
/// ## NOTE
/// Embeds can be nested. That is not handled by this API.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RunEmbeds {
    /// Embeds the full game resource.
    Game,
    /// Embeds the category resource for the run.
    Category,
    /// Embeds the level for the run. This can be empty if it is a full-game
    /// run.
    Level,
    /// Embeds the full user/guest resource in place of the `players` field.
    Players,
    /// Embeds the full region resource. Can be empty if no region was set.
    Region,
    /// Embeds the full platform resource. Can be empty if no platform was set.
    Platform,
}

/// Verification status for the run.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    /// Not yet reviewed.
    New,
    /// Run has been verified by a moderator.
    Verified,
    /// Run has been rejected by a moderator.
    Rejected,
}

/// Sorting options for runs
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum RunsSorting {
    /// Sorts by the game the run was done in (default)
    Game,
    /// Sorts by the run category
    Category,
    /// Sorts by the run level
    Level,
    /// Sorts by the platform used for the run
    Platform,
    /// Sorts by the console region used for the run
    Region,
    /// Sorts by whether an emulator was used for the run
    Emulated,
    /// Sorts by the date of the run
    Date,
    /// Sorts by the date when the run was submitted to speedrun.com
    Submitted,
    /// Sorts by verification status
    Status,
    /// Sorts by the date the run was verified
    VerifyDate,
}

/// Identifies a player (either a user or a guest).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "rel")]
pub enum Player<'a> {
    /// A user.
    User {
        /// `ID` of the user.
        id: UserId<'a>,
    },
    /// A guest.
    Guest {
        /// Name of the guest player.
        name: Cow<'a, str>,
    },
}

/// Represents a [splits.io](https://splits.io) `ID` or URL.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum SplitsIo {
    /// Splits.io ID
    Id(String),
    /// Splits.io URL
    Url(url::Url),
}

// Does this belong here?
/// Type of the variable value.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ValueType<'a> {
    /// Pre-defined variable
    PreDefined {
        /// Value ID
        value: ValueId<'a>,
    },
    /// User defined variable
    UserDefined {
        /// Value ID
        value: ValueId<'a>,
    },
}

/// Updated status for a run.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "status")]
pub enum NewStatus {
    /// Run has been verified.
    Verified,
    /// Run has been rejected.
    Rejected {
        /// The reason the run was rejected (required).
        reason: String,
    },
}

/// Represents a run ID.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct RunId<'a>(Cow<'a, str>);

impl<'a> RunId<'a> {
    /// Create a new [`RunId`].
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for RunId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Display for RunId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Returns a list of all runs.
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Runs<'a> {
    #[doc = r"Return only runs done by `user`."]
    user: Option<UserId<'a>>,
    #[doc = r"Return only runs done by `guest`."]
    guest: Option<Cow<'a, str>>,
    #[doc = r"Return only runs examined by `examiner`."]
    examiner: Option<UserId<'a>>,
    #[doc = r"Restrict results to `game`."]
    game: Option<GameId<'a>>,
    #[doc = r"Restrict results to `level`."]
    level: Option<LevelId<'a>>,
    #[doc = r"Restrict results to `category`."]
    category: Option<CategoryId<'a>>,
    #[doc = r"Restrict results to `platform`."]
    platform: Option<PlatformId<'a>>,
    #[doc = r"Restrict results to `region`."]
    region: Option<RegionId<'a>>,
    #[doc = r"Only return games run on an emulator when `true`."]
    emulated: Option<bool>,
    #[doc = r"Filter runs based on status."]
    status: Option<RunStatus>,
    #[doc = r"Sorting options for results."]
    orderby: Option<RunsSorting>,
    #[doc = r"Sort direction"]
    direction: Option<Direction>,
    #[builder(setter(name = "_embed"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<RunEmbeds>,
}

/// Retrieves a single run.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Run<'a> {
    #[doc = r"`ID` of the run."]
    id: RunId<'a>,
}

/// Submit a new run.
///
/// This endpoint requires a valid API key.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct CreateRun<'a> {
    #[doc = r"Category ID for the run."]
    category: CategoryId<'a>,
    #[doc = r"Level ID for individual level runs."]
    #[builder(default)]
    level: Option<LevelId<'a>>,
    #[doc = r"Optional date the run was performed (defaults to the current date)."]
    #[builder(default)]
    date: Option<Cow<'a, str>>,
    #[doc = r"Optional region for the run. Some games require a region to be submitted."]
    #[builder(default)]
    region: Option<RegionId<'a>>,
    #[doc = r"Optional platform for the run. Some games require a platform to be submitted."]
    #[builder(default)]
    platform: Option<PlatformId<'a>>,
    #[doc = r"If the run has been verified by a moderator. Can only be set if the submitting user is a moderator of the game."]
    #[builder(default)]
    verified: Option<bool>,
    // TODO: Convert this to a private method?
    #[doc = r"Timing information for the run. At least one time must be set."]
    times: Times,
    #[builder(setter(name = "_players"), private, default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    players: Vec<Player<'a>>,
    #[doc = r"When `true` the run was performed on an emulator (default: false)."]
    emulated: Option<bool>,
    #[doc = r"A valid video URL. Optional, but some games require a video to be included."]
    #[builder(default)]
    video: Option<url::Url>,
    #[doc = r"Optional comment on the run. Can include additional video URLs."]
    #[builder(default)]
    comment: Option<String>,
    #[doc = r"Splits.io ID or URL for the splits for the run."]
    #[builder(default)]
    splitsio: Option<SplitsIo>,
    #[doc = r"Variable values for the new run. Some games have mandatory variables."]
    #[builder(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    variables: HashMap<VariableId<'a>, ValueType<'a>>,
}

/// Timing information for a new run.
#[derive(Default, Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Times {
    /// Real-world time of the run.
    realtime: Option<f64>,
    /// Real-world time of the run, excluding the loading times.
    realtime_noloads: Option<f64>,
    /// Time measuered by the game.
    ingame: Option<f64>,
}

/// Update the verification status for the run.
///
/// Requires a valid API key for an authenticated user. The authenticated user
/// must have sufficient permissions (global moderator or game moderator) to
/// change the verification status of a run.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UpdateRunStatus<'a> {
    #[doc = r"`ID` of the run."]
    #[serde(skip)]
    id: RunId<'a>,
    #[doc = r"Updated status for the run."]
    status: NewStatus,
}

/// Change the list of players that participated in a run.
///
/// The updated list must contain at least one player or guest.
///
/// The submitted list of players will replace the old list completely. i.e. you
/// cannot just add a player without also submitting the existing players.
///
/// Requires a valid API key for an authenticated user. The authenticated user
/// must have sufficient permissions (global moderator or game moderator) to
/// change the verification status of a run.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UpdateRunPlayers<'a> {
    #[doc = r"`ID` of the run."]
    #[serde(skip)]
    id: RunId<'a>,
    #[builder(setter(name = "_players"), private)]
    players: Vec<Player<'a>>,
}

/// Delete a run.
///
/// Requires a valid API key for an authenticated user. Regular users can only
/// delete their own runs. Moderators can delete runs by other users also.
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct DeleteRun<'a> {
    #[doc = r"`ID` of the run."]
    id: RunId<'a>,
}

impl<'a> Runs<'a> {
    /// Create a builder for this endpoint.
    pub fn builder() -> RunsBuilder<'a> {
        RunsBuilder::default()
    }
}

impl<'a> RunsBuilder<'a> {
    /// Add an embedded resource to this result
    pub fn embed(&mut self, embed: RunEmbeds) -> &mut Self {
        self.embed.get_or_insert_with(BTreeSet::new).insert(embed);
        self
    }

    /// Add multiple embedded resources to this result
    pub fn embeds<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = RunEmbeds>,
    {
        self.embed.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }
}

impl<'a> Run<'a> {
    /// Create a builder for this endpoint
    pub fn builder() -> RunBuilder<'a> {
        RunBuilder::default()
    }
}

impl<'a> CreateRun<'a> {
    /// Create a builder for this endpoint
    pub fn buider() -> CreateRunBuilder<'a> {
        CreateRunBuilder::default()
    }
}

impl<'a> CreateRunBuilder<'a> {
    /// Add a player to this run.
    pub fn player(&mut self, player: Player<'a>) -> &mut Self {
        self.players.get_or_insert_with(Vec::new).push(player);
        self
    }

    /// Add multiple players to this run.
    pub fn players<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = Player<'a>>,
    {
        self.players.get_or_insert_with(Vec::new).extend(iter);
        self
    }
}

impl<'a> UpdateRunStatus<'a> {
    /// Create a builder for this endpoint
    pub fn builder() -> UpdateRunStatusBuilder<'a> {
        UpdateRunStatusBuilder::default()
    }
}

impl<'a> UpdateRunPlayers<'a> {
    /// Create a builder for this endpoint
    pub fn builder() -> UpdateRunPlayersBuilder<'a> {
        UpdateRunPlayersBuilder::default()
    }
}

impl<'a> UpdateRunPlayersBuilder<'a> {
    /// Add a single user/guest to the updated list of players.
    pub fn player(&mut self, player: Player<'a>) -> &mut Self {
        self.players.get_or_insert_with(Vec::new).push(player);
        self
    }

    /// Add multiple users/guests to the updated list of players.
    pub fn players<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = Player<'a>>,
    {
        self.players.get_or_insert_with(Vec::new).extend(iter);
        self
    }
}

impl<'a> DeleteRun<'a> {
    /// Create a builder for this endpoint
    pub fn builder() -> DeleteRunBuilder<'a> {
        DeleteRunBuilder::default()
    }
}

impl RunEmbeds {
    fn as_str(&self) -> &'static str {
        match self {
            RunEmbeds::Game => "game",
            RunEmbeds::Category => "category",
            RunEmbeds::Level => "level",
            RunEmbeds::Players => "players",
            RunEmbeds::Region => "region",
            RunEmbeds::Platform => "platform",
        }
    }
}

impl Default for RunsSorting {
    fn default() -> Self {
        Self::Game
    }
}

impl Endpoint for Runs<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/runs".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Endpoint for Run<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/runs/{}", self.id).into()
    }
}

impl Endpoint for CreateRun<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/runs".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        Ok(serde_json::to_vec(self).map(|body| Some(("application/json", body)))?)
    }

    fn requires_authentication(&self) -> bool {
        true
    }
}

impl Endpoint for UpdateRunStatus<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/runs/{}/status", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        Ok(serde_json::to_vec(self).map(|body| Some(("application/json", body)))?)
    }

    fn requires_authentication(&self) -> bool {
        true
    }
}

impl Endpoint for UpdateRunPlayers<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/runs/{}/players", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        Ok(serde_json::to_vec(self).map(|body| Some(("application/json", body)))?)
    }

    fn requires_authentication(&self) -> bool {
        true
    }
}

impl Endpoint for DeleteRun<'_> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/runs/{}", self.id).into()
    }

    fn requires_authentication(&self) -> bool {
        true
    }
}

impl From<&RunEmbeds> for &'static str {
    fn from(value: &RunEmbeds) -> Self {
        value.as_str()
    }
}

impl Pageable for Runs<'_> {}
