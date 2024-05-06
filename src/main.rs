pub mod lib;
pub mod models;
pub mod responses;
mod routes;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    get,
    middleware::{Logger, NormalizePath},
    web::{self, Data, Path, Redirect},
    App, Either, HttpResponse, HttpServer, Responder,
};
use dotenvy::{dotenv, var};
use env_logger::Env;
use fs::NamedFile;
use libsql::{Builder, Database};
use models::shortned::Shortned;
use std::sync::Arc;

pub const HOST: &str = "127.0.0.1";
pub const PORT: u16 = 3000;

#[get("/{redirect}")]
async fn redirect(params: Path<String>, db: Data<Arc<Database>>) -> impl Responder {
    let name = params.into_inner();

    let conn = db.connect().unwrap();

    match Shortned::get_by_name(&conn, name).await {
        Ok(res) => match res {
            Some(item) => Either::Left(Redirect::to(item.link)),
            None => Either::Left(Redirect::to("/404")),
        },
        Err(e) => Either::Right(HttpResponse::from_error(e)),
    }
}

#[get("/404")]
async fn not_found() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/404.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to load .env file");
    let url = var("TURSO_DATABASE_URL").expect("LIBSQL_URL must be set");
    let token = var("TURSO_AUTH_TOKEN").expect("LIBSQL_AUTH_TOKEN must be set");

    let db: Arc<libsql::Database> = Arc::new(
        Builder::new_remote(url, token)
            .build()
            .await
            .expect("Connection to database couldn't be made"),
    );

    println!("Listening on http://{HOST}:{PORT}");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(cors)
            .service(fs::Files::new("/admin", "static/").index_file("index.html"))
            .app_data(Data::new(Arc::clone(&db)))
            .service(web::scope("/api").configure(routes::config))
            .service(redirect)
            .service(not_found)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
