use speedrun_api::{
    api::{variables::Variable, AsyncQuery},
    error::SpeedrunApiResult,
    types, SpeedrunApiBuilder,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::default().build_async()?;

    let endpoint = Variable::builder().id("ylpm6vlg").build().unwrap();
    let variable: types::Variable = endpoint.query_async(&client).await?;
    println!("{:#?}", variable);

    Ok(())
}
