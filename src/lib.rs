use actix_web::web;

pub mod config;
pub mod routes;

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(routes::root_config);
}