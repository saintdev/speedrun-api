#![warn(
    future_incompatible,
    rust_2018_compatibility,
    rust_2018_idioms,
    unused,
    missing_docs
)]
#![warn(clippy::all)]
#![allow(rustdoc::broken_intra_doc_links)]

//! This crate implements a wrapper for the Speedrun.com REST API.
//!
//! Endpoints are available in the [api](src/api.rs) module.

#[macro_use]
extern crate derive_builder;

mod auth;
mod client;

pub mod api;
pub mod error;
pub mod types;

pub use auth::AuthError;
pub use client::{SpeedrunApiBuilder, SpeedrunApiClient, SpeedrunApiClientAsync};

//TODO:
//      - Tests
//      - declare_endpoint!() macro?
//      - Endpoint builder errors
