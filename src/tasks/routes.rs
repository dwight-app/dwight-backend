use crate::tasks::{Task, Tasks};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/tasks")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let tasks = web::block(|| Tasks::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(tasks))
}

#[get("/tasks/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let task = Tasks::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(task))
}

#[post("/tasks")]
async fn create(task: web::Json<Task>) -> Result<HttpResponse, CustomError> {
    let task = Tasks::create(task.into_inner())?;
    Ok(HttpResponse::Ok().json(task))
}

#[put("/tasks/{id}")]
async fn update(
    id: web::Path<i32>,
    task: web::Json<Task>,
) -> Result<HttpResponse, CustomError> {
    let task = Tasks::update(id.into_inner(), task.into_inner())?;
    Ok(HttpResponse::Ok().json(task))
}

#[delete("/tasks/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_task = Tasks::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_task })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}