
use std::sync::Mutex;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

// Import the users module
mod users;
mod todo;
mod db;
mod entities;
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
#[warn(unused)]
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
    let env = dotenvy::dotenv().ok();
    println!("{:?}", env);

    let db = web::Data::new(db::connect_db().await);
    println!("Connected to the database: {:?}", db);

    let app_state = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(db.clone())
            .service(index)
            .route("/signup", web::post().to(signup_user))
            .route("/api/v1/update", web::post().to(update_user_name))
            .route("/api/v1/todo", web::post().to(create_todo))
            .route("/api/v1/todo", web::get().to(get_todos))
            .route("/api/v1/todo/{id}", web::put().to(update_todos))
            .route("/api/v1/todo/{id}", web::delete().to(delete_todos))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}