use crate::jobs::{Job, Jobs};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/jobs")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let jobs = Jobs::find_all()?;
    Ok(HttpResponse::Ok().json(jobs))
}

#[get("/jobs/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let job = Jobs::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(job))
}

#[post("/jobs")]
async fn create(job: web::Json<Job>) -> Result<HttpResponse, CustomError> {
    let job = Jobs::create(job.into_inner())?;
    Ok(HttpResponse::Ok().json(job))
}

#[put("/jobs/{id}")]
async fn update(
    id: web::Path<i32>,
    job: web::Json<Job>,
) -> Result<HttpResponse, CustomError> {
    let job = Jobs::update(id.into_inner(), job.into_inner())?;
    Ok(HttpResponse::Ok().json(job))
}

#[delete("/jobs/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_job = Jobs::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_job })))
}

pub fn init_routes(configuration: &mut web::ServiceConfig) {
    configuration.service(find_all);
    configuration.service(find);
    configuration.service(create);
    configuration.service(update);
    configuration.service(delete);
}
