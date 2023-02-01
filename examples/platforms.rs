use std::future;

use futures::TryStreamExt;
use speedrun_api::{
    api::{
        platforms::{Platform, Platforms},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = Platforms::builder().build().unwrap();
    endpoint
        .stream(&client)
        .try_for_each_concurrent(10, |platform: types::Platform| {
            println!("{}", platform.name);
            future::ready(Ok(()))
        })
        .await?;

    // This platform ID does not exist, however this is the example used in the API
    // documentation.
    let endpoint = Platform::builder().id("rdjq4vwe").build().unwrap();
    let platform: types::Platform = endpoint.query_async(&client).await?;
    println!("{platform:#?}");

    Ok(())
}
