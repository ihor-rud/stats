use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::api::AppState;

#[derive(Debug, Deserialize)]
struct Request {
    symbol: String,
    values: Vec<f64>, // data order is from oldest to newest
}

#[derive(Debug, Serialize)]
struct Response {}

#[post("/add_batch")]
async fn add_batch(
    app_state: web::Data<AppState>,
    web::Json(request): web::Json<Request>,
) -> HttpResponse {
    if request.values.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "context": "`values` should not be empty"
        }));
    }

    app_state.state.add_batch(request.symbol, request.values);
    HttpResponse::Ok().json(Response {})
}
