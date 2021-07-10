pub use client::{AsyncClient, Client, RestClient};
pub use common::{CategoriesSorting, Direction, VariablesSorting};
pub use error::ApiError;
pub use pagination::{Pageable, PagedEndpointExt, PagedIter, SinglePage, SinglePageBuilder};
pub use query::AsyncQuery;

pub mod categories;
mod client;
mod common;
mod endpoint;
mod error;
pub mod games;
pub mod guests;
pub mod leaderboards;
pub mod levels;
pub mod notifications;
mod pagination;
pub mod platforms;
//TODO: Authentacation
// pub mod profile;
mod query;
pub mod regions;
pub mod runs;
pub mod series;
pub mod users;
pub mod variables;
