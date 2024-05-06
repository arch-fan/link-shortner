use crate::{
    lib::{jwt, slug::sluggify},
    models::shortned::Shortned,
    responses::ErrResponse,
};
use actix_web::{
    get, post,
    web::{self, Data, Path},
    Responder,
};
use actix_web::{guard, HttpResponse};
use libsql::Database;
use serde::Deserialize;
use std::sync::Arc;

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

#[derive(Deserialize, Clone)]
struct CreateBody {
    name: String,
    link: String,
}

#[post("")]
async fn create_link(body: web::Json<CreateBody>, db: Data<Arc<Database>>) -> impl Responder {
    let conn = db.connect().unwrap();

    let CreateBody { name, link } = body.0;

    match Shortned::create_link(&conn, name, sluggify(link)).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(ErrResponse {
            message: e.to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_link).service(
        web::scope("")
            .guard(guard::fn_guard(jwt::guard))
            .service(create_link),
    );
}
