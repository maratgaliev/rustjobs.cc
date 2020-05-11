use crate::db;
use crate::error_handler::CustomError;
use crate::schema::jobs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "jobs"]
pub struct Job {
    pub title: String,
    pub description: String,
    pub salary: Option<i32>,
    pub currency: String,
    pub apply_url: Option<String>,
    pub job_city: String,
    pub job_email: String,
    pub company: String,
    pub company_twitter: Option<String>,
    pub company_website: String,
    pub company_logo: Option<String>,
    pub slug: Option<String>
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "jobs"]
pub struct Jobs {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub salary: Option<i32>,
    pub currency: String,
    pub apply_url: Option<String>,
    pub job_city: String,
    pub job_email: String,
    pub company: String,
    pub company_twitter: Option<String>,
    pub company_website: String,
    pub company_logo: Option<String>,
    pub slug: String
}

impl Jobs {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let jobs = jobs::table.load::<Jobs>(&conn)?;
        Ok(jobs)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let job = jobs::table.filter(jobs::id.eq(id)).first(&conn)?;
        Ok(job)
    }

    pub fn create(job: Job) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let job = Job::from(job);
        let job = diesel::insert_into(jobs::table)
            .values(job)
            .get_result(&conn)?;
        Ok(job)
    }

    pub fn update(id: i32, job: Job) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let job = diesel::update(jobs::table)
            .filter(jobs::id.eq(id))
            .set(job)
            .get_result(&conn)?;
        Ok(job)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(jobs::table.filter(jobs::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Job {
    fn from(job: Job) -> Job {
        Job {
            title: job.title,
            description: job.description,
            salary: job.salary,
            currency: job.currency,
            apply_url: job.apply_url,
            job_city: job.job_city,
            job_email: job.job_email,
            company: job.company,
            company_twitter: job.company_twitter,
            company_website: job.company_website,
            company_logo: job.company_logo,
            slug: job.slug
        }
    }
}
