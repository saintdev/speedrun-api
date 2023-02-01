use std::future;

use futures::TryStreamExt;
use speedrun_api::{
    api::{
        regions::{Region, Regions},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = Regions::builder().build().unwrap();
    endpoint
        .stream(&client)
        .try_for_each_concurrent(10, |region: types::Region| {
            println!("{}", region.name);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Region::builder().id("pr184lqn").build().unwrap();
    let region: types::Region = endpoint.query_async(&client).await?;
    println!("{region:#?}");

    Ok(())
}
