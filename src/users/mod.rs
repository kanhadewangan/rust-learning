use actix_web::{web, HttpResponse, Responder};
use rand::rngs::OsRng;
use serde::Deserialize;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};

use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use crate::entities::users;


#[derive(Debug, Deserialize)]
pub struct Signup {
    pub username: String,
    pub email: String,
    pub password: String,
}


pub fn hashed_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();
    password_hash
}

pub async fn signup_user(
    db: web::Data<DatabaseConnection>,
    user: web::Json<Signup>,
) -> impl Responder {
    let hashed_password = hashed_password(&user.password);
    let new_user = users::ActiveModel {
        username: Set(user.username.clone()),
        email: Set(user.email.clone()),
        password: Set(hashed_password),
        ..Default::default()
    };
    let res = new_user.insert(db.as_ref()).await;
    match res {
        Ok(_) => {
            println!("User inserted successfully");
        },
        Err(e) => {
            println!("Error inserting user: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to insert user");
        }
    }
    HttpResponse::Ok().body(format!("Signup successful for {}", user.username))
}


pub async  fn update_user_name(user: web::Json<Signup>) -> impl Responder{
    
    println!("hii  find your request we get back to you in few minutes , {}", user.username);
    HttpResponse::Ok().body(format!("get info about it"))
}

