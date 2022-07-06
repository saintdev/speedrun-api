//! API endpoint modules
//!
//! All endpoints use a builder pattern to construct their parameters.
//!
//! # Example
//!
//! ```rust ,no_run
//! use futures::{StreamExt, TryStreamExt};
//! use serde::Deserialize;
//! use speedrun_api::{
//!     api::{self, AsyncQuery, PagedEndpointExt},
//!     error::SpeedrunApiResult,
//!     SpeedrunApiBuilder,
//! };
//!
//! // The return type for the endpoints we are interested in. More information
//! // is returned by the endpoints, but you can deserialize only what is needed.
//! #[derive(Debug, Deserialize)]
//! struct Game {
//!     names: Names,
//!     weblink: String,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Names {
//!     international: String,
//! }
//!
//! #[tokio::main]
//! pub async fn main() -> SpeedrunApiResult<()> {
//!     // Create a new client
//!     let client = SpeedrunApiBuilder::new().build_async()?;
//!
//!     // Create an endpoint. This endpoint gets Super Mario Sunshine.
//!     let endpoint = api::games::Game::builder().id("v1pxjz68").build().unwrap();
//!     // Call the endpoint. The return type decides how to represent the returned value.
//!     let game: Game = endpoint.query_async(&client).await?;
//!
//!     // Create a paginated endpoint. This retrievs a list of all games.
//!     let paginated_endpoint = api::games::Games::builder().build().unwrap();
//!     // The `PagedEndpointExt` adapters consume the endpoint, so we create a copy.
//!     let async_stream = paginated_endpoint.clone();
//!     // Call the `PagedEndpointExt::stream()` method to get an async Stream of results.
//!     let games: Vec<Game> = async_stream.stream(&client).take(200).try_collect().await?;
//!     // Call the `PagedEndpointExt::single_page()` method to retrieve a single page builder.
//!     // This retrieves 100 results starting at offset 100.
//!     let single_page_endpoint = paginated_endpoint
//!         .single_page()
//!         .offset(100)
//!         .max(100)
//!         .build()
//!         .unwrap();
//!     // This wrapped endpoint can be queried like any normal endpoint, but always returns a
//!     // `Vec<T: Deserialize>`.
//!     let games: Vec<Game> = single_page_endpoint.query_async(&client).await?;
//! }
//! ```

mod client;
mod common;
mod endpoint;
mod error;
mod pagination;
mod query;
mod utils;

pub mod categories;
pub mod developers;
pub mod engines;
pub mod games;
pub mod gametypes;
pub mod genres;
pub mod guests;
pub mod leaderboards;
pub mod levels;
pub mod notifications;
pub mod platforms;
pub mod profile;
pub mod publishers;
pub mod regions;
pub mod runs;
pub mod series;
pub mod users;
pub mod variables;

pub use client::{AsyncClient, Client, RestClient};
pub use common::{CategoriesSorting, Direction, VariablesSorting, Root};
pub use error::ApiError;
pub use pagination::{Pageable, PagedEndpointExt, PagedIter, SinglePage, SinglePageBuilder};
pub use query::AsyncQuery;
