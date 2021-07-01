#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::all)]
#![allow(broken_intra_doc_links)]
// TODO: Documentation
//#![warn(missing_docs)]

#[macro_use]
extern crate derive_builder;

mod client;

pub mod api;
pub mod error;

pub use client::{SpeedrunApiClient, SpeedrunApiClientAsync, SpeedrunApiBuilder};

//TODO:
//      - Tests
//      - declare_endpoint!() macro?
//      - Handle embeds?
//      - Docs
//      - Endpoint builder errors
