use std::future;

use futures::{StreamExt, TryStreamExt};
use speedrun_api::{
    api::{
        genres::{Genre, Genres},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = Genres::builder().build().unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |genre: types::Genre| {
            println!("{}", genre.name);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Genre::builder().id("qdnqyk28").build().unwrap();
    let genre: types::Genre = endpoint.query_async(&client).await?;
    println!("{genre:#?}");

    Ok(())
}
