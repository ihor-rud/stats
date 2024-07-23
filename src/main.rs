use actix_web::{web, App, HttpServer};
use api::add_batch::add_batch;
use api::stats::stats;
use api::AppState;

mod api;
mod domain;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    println!("Running on http://127.0.0.1:8080");

    let state = web::Data::new(AppState {
        state: Default::default(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(add_batch)
            .service(stats)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
