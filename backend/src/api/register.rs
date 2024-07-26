use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{AppState, User};

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    name: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    success: bool,
    message: String,
    user: Option<User>,
}

#[post("/register")]
pub async fn register(
    json: web::Json<RegisterRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let users = data.users.lock();
    if users.is_err() {
        return HttpResponse::InternalServerError().json(RegisterResponse {
            success: false,
            message: String::from("Something went wrong! Please try again later."),
            user: None,
        });
    }
    let mut users = users.unwrap();

    let req_body = json.into_inner();

    let record = users.iter().find(|user| user.username == req_body.username);
    if record.is_some() {
        return HttpResponse::Conflict().json(RegisterResponse {
            success: false,
            message: String::from("User with the provided username already exists!"),
            user: None,
        });
    }

    let new_user = User {
        username: req_body.username,
        name: req_body.name,
        password: req_body.password,
    };
    users.push(new_user.clone());

    HttpResponse::Created().json(RegisterResponse {
        success: true,
        message: String::from("Successfully created the user."),
        user: Some(new_user),
    })
}
