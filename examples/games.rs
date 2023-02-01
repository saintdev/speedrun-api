use futures::{StreamExt, TryStreamExt};
use speedrun_api::{
    api::{
        games::{
            Game, GameCategories, GameDerivedGames, GameLevels, GameRecords, GameVariables, Games,
            LeaderboardScope,
        },
        AsyncQuery, PagedEndpointExt,
    },
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::new().build_async()?;

    let endpoint = Games::builder().build().unwrap();
    let games: Vec<types::Game> = endpoint.stream(&client).take(40).try_collect().await?;
    println!("{games:#?}");

    let endpoint = Game::builder().id("v1pxjz68").build().unwrap();
    let game: types::Game = endpoint.query_async(&client).await?;
    println!("{game:#?}");

    let endpoint = Game::builder().id("sms").build().unwrap();
    let game: types::Game = endpoint.query_async(&client).await?;
    println!("{game:#?}");

    let endpoint = GameCategories::builder().id("v1pxjz68").build().unwrap();
    let categories: Vec<types::Category> = endpoint.query_async(&client).await?;
    println!("{categories:#?}");

    let endpoint = GameCategories::builder()
        .id("v1pxjz68")
        .miscellaneous(false)
        .build()
        .unwrap();
    let categories: Vec<types::Category> = endpoint.query_async(&client).await?;
    println!("{categories:#?}");

    let endpoint = GameLevels::builder().id("v1pxjz68").build().unwrap();
    let categories: Vec<types::Level> = endpoint.query_async(&client).await?;
    println!("{categories:#?}");

    let endpoint = GameVariables::builder().id("kyd4pxde").build().unwrap();
    let categories: Vec<types::Variable> = endpoint.query_async(&client).await?;
    println!("{categories:#?}");

    let endpoint = GameDerivedGames::builder().id("pd0wq31e").build().unwrap();
    let derived_games: Vec<types::Game> = endpoint.stream(&client).take(40).try_collect().await?;
    println!("{derived_games:#?}");

    let endpoint = GameRecords::builder()
        .id("pd0wq31e")
        .miscellaneous(false)
        .scope(LeaderboardScope::FullGame)
        .build()
        .unwrap();
    let records: Vec<types::Leaderboard> = endpoint.stream(&client).try_collect().await?;
    println!("{records:#?}");

    Ok(())
}
