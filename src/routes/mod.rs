use actix_web::web;
pub mod index;

pub fn root_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(index::config);
}