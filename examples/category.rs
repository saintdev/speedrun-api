use futures::TryStreamExt;

use speedrun_api::{
    api::{
        categories::{Category, CategoryRecords, CategoryVariables},
        AsyncQuery, Direction, PagedEndpointExt, VariablesSorting,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::new().build_async()?;

    let endpoint = Category::builder().id("nxd1rk8q").build().unwrap();
    let category: types::Category = endpoint.query_async(&client).await?;
    println!("{:#?}", category);

    let mut builder = CategoryVariables::builder();
    let endpoint = builder.id("xd1m7rd8").build().unwrap();
    let variables: Vec<types::Variable> = endpoint.query_async(&client).await?;
    println!("{:#?}", variables);

    let endpoint = builder
        .orderby(VariablesSorting::Mandatory)
        .direction(Direction::Desc)
        .build()
        .unwrap();
    let variables: Vec<types::Variable> = endpoint.query_async(&client).await?;
    println!("{:#?}", variables);

    let endpoint = CategoryRecords::builder().id("wkpjpzjk").build().unwrap();
    let page = endpoint.single_page().build();
    let (records, _): (Vec<types::Leaderboard>, _) = page.query_async(&client).await?;
    println!("{:#?}", records);

    let records: Vec<types::Leaderboard> = endpoint.stream(&client).try_collect().await?;
    println!("{:#?}", records);
    Ok(())
}
