use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todos {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub user_id: String
}

pub struct AppState {
    pub todos: Mutex<Vec<Todos>>
}

pub async fn create_todo(todo: web::Json<Todos>, data: web::Data<AppState>) -> impl Responder {
    // Lock the todos vector and push the new todo
    let mut todos = data.todos.lock().unwrap();
    todos.push(todo.into_inner());
    HttpResponse::Ok().body("Todo created successfully")
}

pub async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    // Lock the todos vector and return the todos
    let todos = data.todos.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

pub async fn update_todos(
    path: web::Path<String>,
    todo: web::Json<Todos>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    for t in todos.iter_mut() {
        if t.id == id || t.id == todo.id {
            t.title = todo.title.clone();
            t.description = todo.description.clone();
            t.completed = todo.completed;
            break;
        }
    }
    HttpResponse::Ok().body("Todo updated successfully")
}

pub async fn delete_todos(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    let len_before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() == len_before {
        HttpResponse::NotFound().body(format!("Todo with id {} not found", id))
    } else {
        HttpResponse::Ok().body("Todo deleted successfully")
    }
}
