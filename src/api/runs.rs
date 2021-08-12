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
/// NOTE: Embeds can be nested. That is not handled by this API.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RunEmbeds {
    Game,
    Category,
    Level,
    Players,
    Region,
    Platform,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    New,
    Verified,
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

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "rel")]
pub enum Player<'a> {
    User { id: UserId<'a> },
    Guest { name: Cow<'a, str> },
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum SplitsIo {
    Id(String),
    Url(url::Url),
}

// Does this belong here?
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum VariableType<'a> {
    PreDefined { value: ValueId<'a> },
    UserDefined { value: ValueId<'a> },
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "status")]
pub enum NewStatus {
    Verified,
    Rejected { reason: String },
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct RunId<'a>(Cow<'a, str>);

impl<'a> RunId<'a> {
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

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Runs<'a> {
    user: Option<UserId<'a>>,
    guest: Option<Cow<'a, str>>,
    examiner: Option<UserId<'a>>,
    game: Option<GameId<'a>>,
    level: Option<LevelId<'a>>,
    category: Option<CategoryId<'a>>,
    platform: Option<PlatformId<'a>>,
    region: Option<RegionId<'a>>,
    emulated: Option<bool>,
    status: Option<RunStatus>,
    orderby: Option<RunsSorting>,
    direction: Option<Direction>,
    #[builder(setter(name = "_embed"), private)]
    #[serde(serialize_with = "super::utils::serialize_as_csv")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    embed: BTreeSet<RunEmbeds>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Run<'a> {
    id: RunId<'a>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct CreateRun<'a> {
    category: CategoryId<'a>,
    #[builder(default)]
    level: Option<LevelId<'a>>,
    #[builder(default)]
    date: Option<Cow<'a, str>>,
    #[builder(default)]
    region: Option<RegionId<'a>>,
    #[builder(default)]
    platform: Option<PlatformId<'a>>,
    #[builder(default)]
    verified: Option<bool>,
    times: Times,
    #[builder(setter(name = "_players"), private, default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    players: Vec<Player<'a>>,
    #[builder(default)]
    emulated: Option<bool>,
    #[builder(default)]
    video: Option<url::Url>,
    #[builder(default)]
    comment: Option<String>,
    #[builder(default)]
    splitsio: Option<SplitsIo>,
    #[builder(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    variables: HashMap<VariableId<'a>, VariableType<'a>>,
}

#[derive(Default, Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Times {
    realtime: Option<f64>,
    realtime_noloads: Option<f64>,
    ingame: Option<f64>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UpdateRunStatus<'a> {
    #[serde(skip)]
    id: RunId<'a>,
    status: NewStatus,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UpdateRunPlayers<'a> {
    #[serde(skip)]
    id: RunId<'a>,
    #[builder(setter(name = "_players"), private)]
    players: Vec<Player<'a>>,
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct DeleteRun<'a> {
    id: RunId<'a>,
}

impl<'a> Runs<'a> {
    pub fn builder() -> RunsBuilder<'a> {
        RunsBuilder::default()
    }
}

impl<'a> RunsBuilder<'a> {
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

impl<'a> Run<'a> {
    pub fn builder() -> RunBuilder<'a> {
        RunBuilder::default()
    }
}

impl<'a> CreateRun<'a> {
    pub fn buider() -> CreateRunBuilder<'a> {
        CreateRunBuilder::default()
    }
}

impl<'a> CreateRunBuilder<'a> {
    pub fn player(&mut self, player: Player<'a>) -> &mut Self {
        self.players.get_or_insert_with(Vec::new).push(player);
        self
    }

    pub fn players<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = Player<'a>>,
    {
        self.players.get_or_insert_with(Vec::new).extend(iter);
        self
    }
}

impl<'a> UpdateRunStatus<'a> {
    pub fn builder() -> UpdateRunStatusBuilder<'a> {
        UpdateRunStatusBuilder::default()
    }
}

impl<'a> UpdateRunPlayers<'a> {
    pub fn builder() -> UpdateRunPlayersBuilder<'a> {
        UpdateRunPlayersBuilder::default()
    }
}

impl<'a> UpdateRunPlayersBuilder<'a> {
    pub fn player(&mut self, player: Player<'a>) -> &mut Self {
        self.players.get_or_insert_with(Vec::new).push(player);
        self
    }

    pub fn players<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = Player<'a>>,
    {
        self.players.get_or_insert_with(Vec::new).extend(iter);
        self
    }
}

impl<'a> DeleteRun<'a> {
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
