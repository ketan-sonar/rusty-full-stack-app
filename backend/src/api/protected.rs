use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};
use serde::Serialize;

use crate::{AppState, Claims};

#[derive(Serialize)]
struct ProtectedResponse {
    success: bool,
    message: String,
    username: Option<String>,
}

#[get("/protected")]
pub async fn protected(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let token_cookie = match req.cookie("token") {
        Some(token_cookie) => token_cookie,
        None => {
            return HttpResponse::Unauthorized().json(ProtectedResponse {
                success: false,
                message: String::from("Token Required"),
                username: None,
            })
        }
    };

    match decode::<Claims>(
        token_cookie.value(),
        &DecodingKey::from_secret(data.jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(decoded_token) => HttpResponse::Ok().json(ProtectedResponse {
            success: true,
            message: String::from("Successfully fetched your details."),
            username: Some(decoded_token.claims.username),
        }),
        Err(err) => match err.kind() {
            ErrorKind::InvalidToken => HttpResponse::Unauthorized().json(ProtectedResponse {
                success: false,
                message: String::from("Invalid Token!"),
                username: None,
            }),
            ErrorKind::ExpiredSignature => HttpResponse::Unauthorized().json(ProtectedResponse {
                success: false,
                message: String::from("Token Expired!"),
                username: None,
            }),
            _ => HttpResponse::Unauthorized().json(ProtectedResponse {
                success: false,
                message: String::from("User not logged in!"),
                username: None,
            }),
        },
    }
}
