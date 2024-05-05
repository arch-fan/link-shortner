use actix_web::{web, HttpResponse};
mod link;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(link::config);
}
