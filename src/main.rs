
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::Json};
use serde::Deserialize;



// Import the users module
mod users;

// Import the signup_user function from the users module
use users::signup_user;




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
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/signup", actix_web::web::post().to(signup_user))
            
    })
    .bind("127.0.0.1:8080")?.run().await
}