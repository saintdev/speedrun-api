use std::future;

use futures::{StreamExt, TryStreamExt};
use speedrun_api::{
    api::{
        gametypes::{GameType, GameTypes},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = GameTypes::builder().build().unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |game_type: types::GameType| {
            println!("{}", game_type.name);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = GameType::builder().id("d91jd1ex").build().unwrap();
    let game_type: types::GameType = endpoint.query_async(&client).await?;
    println!("{game_type:#?}");

    Ok(())
}
