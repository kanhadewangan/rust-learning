
use std::sync::Mutex;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;



// Import the users module
mod users;
mod todo;

// Import the signup_user function from the users module
use users::signup_user;
use users::update_user_name;


use todo::{create_todo, get_todos, update_todos, delete_todos, AppState};





#[allow(dead_code)]
struct OrderBook{
    id : String,
    price : f64,
    quantity : f64,
    symbol : String,
    slug : String,
    user_id : String,


}

fn matching(email: String, password: String )-> bool{
    if email == "admin@example.com" && password == "admin123" {
        return true;
    }
    if email == password {return false};

    if email.contains("@") && password.len() >= 8 {
        return true;
    }
    false
}




#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    let app_state = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });
    HttpServer::new(move || {
        App::new().app_data(app_state.clone())
            .service(index)
            .route("/signup", actix_web::web::post().to(signup_user))
            .route("/api/v1/update", actix_web::web::post().to(update_user_name))
            .route("/api/v1/todo", actix_web::web::post().to(create_todo))
            .route("/api/v1/todo", actix_web::web::get().to(get_todos))
            .route("/api/v1/todo/{id}", actix_web::web::put().to(update_todos))
            .route("/api/v1/todo/{id}", actix_web::web::delete().to(delete_todos))
    })
    .bind("127.0.0.1:8080")?.run().await
}