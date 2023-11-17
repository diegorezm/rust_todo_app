mod db;
mod models;

use actix_web::{delete, put, web};
use actix_web::{get, post, web::Json, HttpResponse};

use crate::models::request::Request;
use crate::models::todos::Todo;
use db::Database;

#[get("/todo")]
pub async fn get_todos() -> HttpResponse {
    let db = Database::new().expect("Failed to initialize the database");
    match db.get_all_todos() {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("/todo/add")]
pub async fn add_todo(new_todo: Json<Request>) -> HttpResponse {
    let db = Database::new().expect("Failed to initialize the database");
    match db.create_todo(new_todo.into_inner()) {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[put("/todo/toggle/{id}")]
pub async fn toggle_todo(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let db = Database::new().expect("Failed to initialize the database");
    if let Err(err) = db.toggle_completed(&id).await {
        HttpResponse::InternalServerError().json(err.to_string())
    } else {
        HttpResponse::Ok().body("Toggled.")
    }
}

#[put("/todo")]
pub async fn edited_todo(edited_todo: web::Json<Todo>) -> HttpResponse {
    let db = Database::new().expect("Failed to initialize the database");
    match db.edit_todo(&edited_todo.into_inner()) {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}


#[delete("/todo/{id}")]
pub async fn delete_todo(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let db = Database::new().expect("Failed to initialize the database");
    match db.delete_todo(&id) {
        Ok(resp) => HttpResponse::Ok().body(resp),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(add_todo)
            .service(get_todos)
            .service(delete_todo)
            .service(toggle_todo)
            .service(edited_todo)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
