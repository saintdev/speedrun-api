use std::future;

use futures::{StreamExt, TryStreamExt};
use speedrun_api::{
    api::{
        developers::{Developer, Developers},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = Developers::builder().build().unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |developer: types::Developer| {
            println!("{}", developer.name);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Developer::builder().id("l4eprzro").build().unwrap();
    let developer: types::Developer = endpoint.query_async(&client).await?;
    println!("{developer:#?}");

    Ok(())
}
