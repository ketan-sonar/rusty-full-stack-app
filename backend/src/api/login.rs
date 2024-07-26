use actix_web::{cookie::Cookie, post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{AppState, Claims};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

#[post("/login")]
pub async fn login(json: web::Json<LoginRequest>, data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock();
    if users.is_err() {
        return HttpResponse::InternalServerError().json(LoginResponse {
            success: false,
            message: String::from("Something went wrong! Please try again later."),
        });
    }
    let users = users.unwrap();

    let req_body = json.into_inner();

    let record = users.iter().find(|user| user.username == req_body.username);
    if record.is_none() {
        return HttpResponse::Conflict().json(LoginResponse {
            success: false,
            message: String::from("User with the provided username does not exists!"),
        });
    }
    let record = record.unwrap();
    if record.password != req_body.password {
        return HttpResponse::Ok().json(LoginResponse {
            success: false,
            message: String::from("Wrong password!"),
        });
    }

    let claims = Claims {
        username: req_body.username,
        exp: (Utc::now() + Duration::seconds(30)).timestamp() as usize,
    };
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.jwt_secret.as_ref()),
    ) {
        Ok(token) => HttpResponse::Ok()
            .cookie(
                Cookie::build("token", token)
                    .domain("localhost")
                    .path("/")
                    // .secure(true)
                    .http_only(true)
                    .finish(),
            )
            .json(LoginResponse {
                success: true,
                message: String::from("Successfully logged in."),
            }),
        Err(err) => HttpResponse::InternalServerError().json(LoginResponse {
            success: false,
            message: err.to_string(),
        }),
    }
}
