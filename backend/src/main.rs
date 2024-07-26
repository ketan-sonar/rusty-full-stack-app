use std::io;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use api::{login::login, protected::protected, register::register};
use serde::{Deserialize, Serialize};

mod api;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 3001;

#[derive(Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: usize,
}

#[derive(Clone, Serialize)]
struct User {
    username: String,
    name: String,
    password: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
    jwt_secret: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Mutex::new(vec![]),
        jwt_secret: String::from("secret"),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_state.clone())
            .service(register)
            .service(login)
            .service(protected)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
