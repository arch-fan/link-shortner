use actix_web::{post, web, HttpResponse, Responder};
use dotenvy::var;
use serde::Deserialize;

use crate::lib::jwt;

#[derive(Deserialize)]
struct LoginBody {
    username: String,
    password: String,
}

#[post("")]
async fn login(body: web::Json<LoginBody>) -> impl Responder {
    let username = var("USERNAME").unwrap();
    let password = var("PASSWORD").unwrap();

    if username != body.username || password != body.password {
        return HttpResponse::Unauthorized().finish();
    }

    let LoginBody { username, .. } = body.0;

    let token = jwt::encode(username);

    let mut res = HttpResponse::Ok();
    res.append_header((
        "Set-Cookie",
        format!(
            "{}={}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}; {}",
            jwt::COOKIE_NAME,
            token,
            jwt::get_expiration(),
            if std::env::var("RUST_ENV").unwrap_or("DEV".to_string()) == "DEV" {
                ""
            } else {
                "Secure;"
            }
        ),
    ));
    res.finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
