use std::future;

use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use speedrun_api::{
    api::{
        series::{ListSeries, Series, SeriesGames, SeriesOrderBy},
        AsyncQuery, Direction, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async().await?;

    let endpoint = ListSeries::builder().build().unwrap();
    endpoint
        .stream(&client)
        .take(20)
        .try_for_each_concurrent(10, |series: types::Series| {
            println!("{}", series.names.international);
            future::ready(Ok(()))
        })
        .await?;
    let endpoint = ListSeries::builder()
        .orderby(SeriesOrderBy::Created)
        .direction(Direction::Desc)
        .build()
        .unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(2, |series: types::Series| {
            println!("{}", series.names.international);
            future::ready(Ok(()))
        })
        .await?;
    let endpoint = ListSeries::builder().name("mario").build().unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(2, |series: types::Series| {
            println!("{}", series.names.international);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Series::builder().id("rv7emz49").build().unwrap();
    let series: types::Series = endpoint.query_async(&client).await?;
    println!("{:#?}", series);

    let mut builder = SeriesGames::builder();
    let endpoint = builder.id("rv7emz49").build().unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |game: types::Game| {
            println!("{}", game.names.international);
            future::ready(Ok(()))
        })
        .await?;
    {
        let mut builder = builder.clone();
        let endpoint = builder.released(2003).build().unwrap();
        endpoint
            .stream(&client)
            .take(10)
            .try_for_each_concurrent(5, |game: types::Game| {
                println!("{}", game.names.international);
                future::ready(Ok(()))
            })
            .await?;
    }
    #[derive(Debug, Deserialize)]
    struct BulkGame {
        id: String,
        names: types::Names,
        abbreviation: String,
        weblink: String,
    }
    let endpoint = builder.bulk(true).build().unwrap();
    endpoint
        .stream(&client)
        .try_for_each_concurrent(10, |game: BulkGame| {
            println!("{}", game.names.international);
            future::ready(Ok(()))
        })
        .await?;

    Ok(())
}
