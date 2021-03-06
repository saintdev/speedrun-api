#![allow(missing_docs)]

mod category;
mod category_impls;
mod common;
mod developer_impls;
mod developers;
mod engine_impls;
mod engines;
mod game_impls;
mod games;
mod gametype_impls;
mod gametypes;
mod genre_impls;
mod genres;
mod guest_impls;
mod guests;
mod leaderboard_impls;
mod leaderboards;
mod levels;
mod levels_impls;
mod notifications;
mod platform_impls;
mod platforms;
mod publisher_impls;
mod publishers;
mod region_impls;
mod regions;
mod run_impls;
mod runs;
mod series;
mod series_impls;
mod user_impls;
mod users;
mod variable_impls;
mod variables;

// TODO: Deserialize to URI/URL type
// TODO: Deserialize dates to chrono types

pub use category::{Category, CategoryType, Players};
pub use common::{Asset, Assets, Link, ModeratorRole, Names, Pagination, TimingMethod, Root};
pub use developers::Developer;
pub use engines::Engine;
pub use games::{Game, Ruleset};
pub use gametypes::GameType;
pub use genres::Genre;
pub use guests::Guest;
pub use leaderboards::{Leaderboard, RankedRun};
pub use levels::Level;
pub use platforms::Platform;
pub use publishers::Publisher;
pub use regions::Region;
pub use runs::{Player, Run, Status, System, Times, VideoLink, Videos};
pub use series::Series;
pub use users::{BasicLink, Color, Location, NameStyle, Place, User, UserRole};
pub use variables::{Flags, Scope, Value, Values, Variable};
