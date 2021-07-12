use std::future;

use futures::{StreamExt, TryStreamExt};
use speedrun_api::{
    api::{
        publishers::{Publisher, Publishers},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = Publishers::builder().build().unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |publisher: types::Publisher| {
            println!("{}", publisher.name);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Publisher::builder().id("1z6qgr9p").build().unwrap();
    let publisher: types::Publisher = endpoint.query_async(&client).await?;
    println!("{:#?}", publisher);

    Ok(())
}
