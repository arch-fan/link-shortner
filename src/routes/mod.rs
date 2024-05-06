use actix_web::web::{scope, ServiceConfig};
mod link;
mod links;
mod login;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/link").configure(link::config))
        .service(scope("/links").configure(links::config))
        .service(scope("/login").configure(login::config));
}
