use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use dotenvy::{dotenv, var};
use libsql::Builder;

pub mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to load .env file");
    let url = var("TURSO_DATABASE_URL").expect("LIBSQL_URL must be set");
    let token = var("TURSO_AUTH_TOKEN").expect("LIBSQL_AUTH_TOKEN must be set");

    let db = Builder::new_remote(url, token)
        .build()
        .await
        .expect("Connection to database couldn't be made");

    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 3000;

    println!("Listening on http://{HOST}:{PORT}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::new(db)))
            .service(web::scope("/api").configure(routes::config))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
