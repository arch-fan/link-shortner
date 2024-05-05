use crate::{responses::ErrResponse, Shortned};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use libsql::Database;
use std::sync::Arc;

#[get("")]
async fn get_links(db: Data<Arc<Database>>) -> impl Responder {
    let conn = db.connect().unwrap();

    match Shortned::get_all_links(&conn).await {
        Ok(links) => HttpResponse::Ok().json(links),
        Err(e) => HttpResponse::InternalServerError().json(ErrResponse {
            message: e.to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_links);
}
