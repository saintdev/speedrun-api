use std::future;

use futures::{StreamExt, TryStreamExt};
use speedrun_api::{
    api::{
        runs::{Run, RunStatus, Runs, RunsSorting},
        AsyncQuery, Direction, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = Runs::builder().build().unwrap();
    endpoint
        .stream(&client)
        .take(30)
        .try_for_each_concurrent(10, |run: types::Run| {
            println!("{}", run.weblink);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Runs::builder()
        .status(RunStatus::Verified)
        .orderby(RunsSorting::VerifyDate)
        .direction(Direction::Desc)
        .build()
        .unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |run: types::Run| {
            println!("{}", run.weblink);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Runs::builder()
        .status(RunStatus::New)
        .orderby(RunsSorting::Submitted)
        .direction(Direction::Desc)
        .build()
        .unwrap();
    endpoint
        .stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |run: types::Run| {
            println!("{}", run.weblink);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Runs::builder().guest("Alex").build().unwrap();
    let _ = endpoint
        .stream(&client)
        .try_for_each(|run: types::Run| {
            println!("{}", run.weblink);
            future::ready(Ok(()))
        })
        .await;

    let endpoint = Runs::builder()
        .emulated(true)
        .examiner("wzx7q875")
        .build()
        .unwrap();
    endpoint
        .stream(&client)
        .try_for_each_concurrent(10, |run: types::Run| {
            println!("{}", run.weblink);
            future::ready(Ok(()))
        })
        .await?;

    let endpoint = Run::builder().id("90y6pm7e").build().unwrap();
    let run: types::Run = endpoint.query_async(&client).await?;
    println!("{run:#?}");

    Ok(())
}
