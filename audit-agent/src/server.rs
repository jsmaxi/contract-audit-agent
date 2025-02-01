use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{
    error::ErrorNotFound, get, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    name: String,
}

#[post("/users")]
async fn create_user(user_data: web::Json<User>, db: web::Data<UserDb>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    db.insert(new_id, user_data.into_inner());
    HttpResponse::Created().json(CreateUserResponse { id: new_id, name })
}

#[get("/users/{id}")]
async fn get_user(user_id: web::Path<u32>, db: web::Data<UserDb>) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();
    match db.get(&user_id) {
        Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        None => Err(ErrorNotFound("User not found")),
    }
}

#[get("/greet/{id}")]
async fn greet_user(user_id: web::Path<u32>) -> impl Responder {
    format!("Hello {user_id}")
}

#[get("/greet")]
async fn greet() -> impl Responder {
    format!("Hello World")
}

// #[actix_web::main]
pub async fn serve() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {port}");

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(greet)
            .service(greet_user)
            .service(get_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}
