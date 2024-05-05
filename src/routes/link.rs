use std::sync::Arc;

use crate::{models::shortned::Shortned, responses::ErrResponse};
use actix_web::HttpResponse;
use actix_web::{
    get,
    web::{self, Data, Path},
    Responder,
};
use libsql::Database;

#[get("/{name}")]
async fn get_link(params: Path<String>, db: Data<Arc<Database>>) -> impl Responder {
    let name = params.into_inner();

    let conn = db.connect().unwrap();

    let res = Shortned::get_by_name(&conn, name).await;

    if let Err(e) = res {
        return HttpResponse::InternalServerError().json(ErrResponse {
            message: e.to_string(),
        });
    }

    let row = res.unwrap();

    match row {
        Some(row) => HttpResponse::Ok().json(row),
        None => HttpResponse::NotFound().finish(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_link);
}
