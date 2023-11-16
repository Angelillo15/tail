use actix_web::{get, HttpResponse, web};
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index_get);
}

#[get("/")]
pub async fn index_get() -> HttpResponse {
    let data = json!({
        "name": "tail",
        "version": env!("CARGO_PKG_VERSION"),
        "about": "Welcome to tail!",
    });

    HttpResponse::Ok().json(data)
}