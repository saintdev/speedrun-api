use std::future;

use futures::{StreamExt, TryStreamExt};
use speedrun_api::{
    api::{
        engines::{Engine, Engines},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = Engines::builder().build().unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |engine: types::Engine| {
            println!("{}", engine.name);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Engine::builder().id("p85eo036").build().unwrap();
    let engine: types::Engine = endpoint.query_async(&client).await?;
    println!("{:#?}", engine);

    Ok(())
}
