use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2};

#[derive(Debug, Deserialize)]
pub struct Signup {
    pub username: String,
    pub email: String,
    pub password: String,
}


pub fn hashedPassword(password:&str)->String{
    const SALT: &str = "randomsalt"; // In a real
    let salt_string: SaltString = SaltString::b64_encode(SALT.as_bytes()).unwrap();
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt_string)
        .expect("Failed to hash password")
        .to_string();
    password_hash

}

pub async fn signup_user(user: web::Json<Signup>) -> impl Responder {
    println!("Received signup request: {:?}", user);
    let hashed_password = hashedPassword(&user.password);
    println!("Hashed password: {}", hashed_password);
    HttpResponse::Ok().body(format!("Signup successful for {}", user.username))
}


pub async  fn update_user_name(user: web::Json<Signup>) -> impl Responder{
    
    println!("hii  find your request we get back to you in few minutes , {}", user.username);
    HttpResponse::Ok().body(format!("get info about it"))
}

