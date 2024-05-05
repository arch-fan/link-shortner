use crate::models::shortned::Shortned;
use actix_web::{get, web, Responder};
use libsql::Connection;
use serde::Serialize;

#[derive(Serialize)]
struct OkResponse<T: Serialize> {
    status: String,
    body: T,
}

#[get("/{name}")]
async fn get_link(params: web::Path<String>, db: web::Data<Connection>) -> impl Responder {
    let name = params.into_inner();

    let link = Shortned::get_by_name(&db, name).await;

    web::Json(OkResponse {
        status: "ok".to_string(),
        body: link,
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/link").service(get_link));
}
