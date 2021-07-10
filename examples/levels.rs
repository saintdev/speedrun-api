use futures::{StreamExt, TryStreamExt};
use speedrun_api::{
    api::{
        levels::{Level, LevelCategories, LevelRecords, LevelVariables},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    // This level ID does not exist, however this is the example used in the API
    // documentation.
    let endpoint = Level::builder().id("329vpn9v").build().unwrap();
    let level: types::Level = endpoint.query_async(&client).await?;
    println!("{:#?}", level);

    // This level ID does not exist, however this is the example used in the API
    // documentation.
    let mut builder = LevelCategories::builder();
    builder.id("329vpn9v");
    let endpoint = builder.build().unwrap();
    let categories: Vec<types::Category> = endpoint.query_async(&client).await?;
    println!("{:#?}", categories);
    let endpoint = builder.miscellaneous(false).build().unwrap();
    let categories: Vec<types::Category> = endpoint.query_async(&client).await?;
    println!("{:#?}", categories);

    let endpoint = LevelVariables::builder().id("495ggmwp").build().unwrap();
    let variables: Vec<types::Variable> = endpoint.query_async(&client).await?;
    println!("{:#?}", variables);

    // This level ID does not exist, however this is the example used in the API
    // documentation.
    let endpoint = LevelRecords::builder().id("rdnyx79m").build().unwrap();
    let records: Vec<types::Leaderboard> = endpoint.stream(&client).take(20).try_collect().await?;
    println!("{:#?}", records);

    Ok(())
}
