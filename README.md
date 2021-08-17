Speedrun-Api | A Rust wrapper for the [Speedrun.com API](https://github.com/speedruncomorg/api)
===============================================================================================

Endpoints are available under the [api](src/api.rs) module. Endpoints can be
constructed using a "builder" pattern. To use an endpoint you can query it
using the [`Query`](src/api/query.rs) or [`AsyncQuery`](src/api/query.rs)
traits. Paginated endpoints can be queried using the methods on the 
[`PagedEndpointExt`](src/api/pagination.rs) trait.

All endpoints return types of the caller's choosing that implement the `serde`
`Deserialize` trait. Callers are suggested to define their own types for
obtaining data from the API. This gives more control to the caller on the
exact fields that get deserialized, along with being more adaptable to possible
api changes. Sample types ARE provided in the [types](src/types.rs) module.

### Async Example
-----------------

```rust ,norun
use speedrun_api::{
    api::{
        leaderboards::FullGameLeaderboard,
        AsyncQuery,
    },
    error::SpeedrunApiResult,
    SpeedrunApiBuilder,
    types,
}

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    // Create a new async client
    let client = SpeedrunApiBuilder::new().build_async()?;?;

    // Build a new endpoint
    // This gets the "All Campaigns" leaderboard for Age of Empires II.
    let endpoint = FullGameLeaderboard::builder()
        .game("xldev513")
        .category("rklg3rdn")
        .build()
        .unwrap();

    // Query the endpoint using `client`
    let leaderboard: types::Leaderboard = endpoint.query_async(&client).await?;
}
```

See [examples](./examples) for more examples. Including paginated endpoints.

## Design Notes

The design is based on the blog post [Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is)
by Ben Boeckel, and the [gitlab crate](https://gitlab.kitware.com/utils/rust-gitlab).
