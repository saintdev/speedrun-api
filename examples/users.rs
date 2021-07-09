use std::future;

use futures::{StreamExt, TryStreamExt};

use speedrun_api::{
    api::{
        users::{User, UserPersonalBests, Users},
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async().await?;

    // This should return all users, but the API returns an error. However, this is
    // the example used in the API documentation.
    let endpoint = Users::builder().build().unwrap();
    let _: Result<Vec<types::User>, _> = endpoint.stream(&client).take(40).try_collect().await;

    let endpoint = Users::builder().name("abc").build().unwrap();
    endpoint
        .stream(&client)
        .take(20)
        .try_for_each_concurrent(10, |user: types::User| {
            println!("{}", user.names.international);
            future::ready(Ok(()))
        })
        .await?;

    // This example does not return any results, however this is the example used in
    // the API documentation.
    let endpoint = Users::builder().lookup("pac____").build().unwrap();
    let _: Result<Vec<types::User>, _> = endpoint.stream(&client).take(1).try_collect().await;

    let endpoint = User::builder().id("wzx7q875").build().unwrap();
    let user: types::User = endpoint.query_async(&client).await?;
    println!("{:#?}", user);

    let endpoint = UserPersonalBests::builder().id("wzx7q875").build().unwrap();
    let runs: Vec<types::RankedRun> = endpoint.query_async(&client).await?;
    println!("{:#?}", runs);

    Ok(())
}
