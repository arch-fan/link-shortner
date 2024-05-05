use actix_web::web;
mod link;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(link::config);
}
