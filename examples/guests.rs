use speedrun_api::{
    api::{guests::Guest, AsyncQuery},
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::new().build_async().await?;

    // There is no guest named "Alex", however this is the example used by the API
    // documentation.
    let endpoint = Guest::builder().name("Alex").build().unwrap();
    let guest: types::Guest = endpoint.query_async(&client).await?;
    println!("{:#?}", guest);

    Ok(())
}
