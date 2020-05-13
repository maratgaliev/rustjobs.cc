use crate::jobs::{Job, Jobs};
use crate::error_handler::CustomError;
use actix_web::{get, post, web, HttpResponse};

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

pub fn init_routes(configuration: &mut web::ServiceConfig) {
    configuration.service(find_all);
    configuration.service(find);
    configuration.service(create);
}
