use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use std::env;
use dotenv::dotenv;
use time::{Duration, OffsetDateTime};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: i32,
    description: String,
    done: bool,
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn login(body: web::Json<LoginRequest>) -> impl Responder {
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

fn auth_ok(hdrs: &actix_web::http::header::HeaderMap) -> bool {
    let Some(v) = hdrs.get("Authorization") else { return false; };
    let Ok(s) = v.to_str() else { return false; };
    if !s.starts_with("Bearer ") { return false; }
    let token = &s[7..];
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "devsecret".into());
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256)).is_ok()
}

async fn list_tasks(req: HttpRequest) -> impl Responder {
    let hdrs = req.headers();
    if !auth_ok(hdrs) {
        return HttpResponse::Unauthorized().finish();
    }
    HttpResponse::Ok().json(vec![
        Task { id: 1, description: "Example task".into(), done: false }
    ])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Use DATABASE_URL env or default to CNPG in-cluster service
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:secret@todo-postgres.default.svc.cluster.local:5432/todo".into());

    // Try to connect with a timeout (SQLx 0.7 uses acquire_timeout instead of connect_timeout)
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    println!("Connected to Postgres at {}", database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health))
            .route("/login", web::post().to(login))
            .route("/tasks", web::get().to(list_tasks))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}

