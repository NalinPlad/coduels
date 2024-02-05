use eyre::Context;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use coduels::{app::App, models::_entities::problems};
use coduels::models::_entities::problems::Entity;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ctx = playground::<App>().await.context("playground")?;

    // let active_model: articles::ActiveModel = ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    
    let res = problems::Entity::find().all(&_ctx.db).await?;
    println!("{:?}", res);
    // println!("welcome to playground. edit me at `examples/playground.rs`");

    Ok(())
}
