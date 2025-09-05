use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::models::{LoginRequest, TokenResponse, Task, Claims};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use std::env;
use time::{Duration, OffsetDateTime};

fn auth_ok(hdrs: &actix_web::http::header::HeaderMap) -> bool {
    let Some(v) = hdrs.get(actix_web::http::header::AUTHORIZATION) else { return false; };
    let Ok(s) = v.to_str() else { return false; };
    if !s.starts_with("Bearer ") { return false; }
    let token = &s[7..];
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "devsecret".into());
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256)).is_ok()
}

pub async fn login(body: web::Json<LoginRequest>) -> impl Responder {
    if body.username != "admin" || body.password != "password" {
        return HttpResponse::Unauthorized().finish();
    }

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "devsecret".into());
    let exp = (OffsetDateTime::now_utc() + Duration::hours(1)).unix_timestamp() as usize;
    let claims = Claims { sub: body.username.clone(), exp };
    let token = encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .expect("token encode");

    HttpResponse::Ok().json(TokenResponse { access_token: token, token_type: "Bearer".into() })
}

pub async fn list_tasks(req: HttpRequest) -> impl Responder {
    let hdrs = req.headers();

    if !auth_ok(hdrs) {
        return HttpResponse::Unauthorized().finish();
    }

    HttpResponse::Ok().json(vec![
        Task { id: 1, description: "Example task".into(), done: false }
    ])
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/login", web::post().to(login))
        .route("/tasks", web::get().to(list_tasks));
}
