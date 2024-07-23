use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::api::AppState;
use crate::domain::models::WindowSize;

#[derive(Debug, Deserialize)]
struct Request {
    symbol: String,
    k: u8,
}

#[derive(Debug, Serialize)]
struct Response {
    min: f64,
    max: f64,
    last: f64,
    avg: f64,
    var: f64,
}

#[get("/stats")]
async fn stats(
    app_state: web::Data<AppState>,
    web::Query(request): web::Query<Request>,
) -> HttpResponse {
    let window_size: WindowSize = match request.k.try_into() {
        Ok(window_size) => window_size,
        Err(err) => {
            return HttpResponse::BadRequest().json(json!({
                "context": err.context("invalid `k` parameter").to_string()
            }))
        }
    };

    let result = app_state.state.get_stats(&request.symbol, window_size);

    match result {
        Ok(stats) => HttpResponse::Ok().json(Response {
            min: stats.min,
            max: stats.max,
            last: stats.last,
            avg: stats.avg,
            var: stats.var,
        }),
        Err(err) => HttpResponse::BadRequest().json(json!({
            "context": err.to_string()
        })),
    }
}
