#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use crate::models::_entities::problems::{self, ActiveModel, Entity, Model};

// pub async fn echo(req_body: String) -> String {
//     req_body
// }

// pub async fn hello(State(_ctx): State<AppContext>) -> Result<String> {
//     // do something with context (database, etc)
//     format::text("hello")
// }

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    let problems = problems::Entity::find().all(&ctx.db).await?;
    Ok(Json(problems))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("problems")
        // .add("/", get(hello))
        // .add("/echo", post(echo))
        .add("/list", get(list))
}
