use crate::db;
use crate::error_handler::CustomError;
use crate::schema::jobs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::prelude::{Utc};
use std::fmt;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "jobs"]
pub struct Job {
    pub title: String,
    pub description: String,
    pub salary: Option<i32>,
    pub job_type: String,
    pub is_remote: Option<bool>,
    pub currency: String,
    pub apply_url: Option<String>,
    pub job_city: String,
    pub job_email: Option<String>,
    pub company: String,
    pub company_twitter: Option<String>,
    pub company_website: String,
    pub company_logo: Option<String>,
    pub slug: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "jobs"]
pub struct Jobs {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub salary: Option<i32>,
    pub job_type: String,
    pub is_remote: Option<bool>,
    pub currency: String,
    pub apply_url: Option<String>,
    pub job_city: String,
    pub job_email: Option<String>,
    pub company: String,
    pub company_twitter: Option<String>,
    pub company_website: String,
    pub company_logo: Option<String>,
    pub slug: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

// DEBUG PURPOSES
impl fmt::Debug for Job {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Job")
       .field("created_at", &self.created_at)
       .finish()
  }
}

impl Jobs {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let jobs = jobs::table.select(jobs::all_columns).order(jobs::created_at.desc()).load::<Jobs>(&conn)?;
        Ok(jobs)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let job = jobs::table.filter(jobs::id.eq(id)).first(&conn)?;
        Ok(job)
    }

    pub fn create(job: Job) -> Result<Self, CustomError> {
        let now = Utc::now().naive_utc();
        let job = Job {
          created_at: Some(now),
          updated_at: Some(now),
          ..job
        };

        let conn = db::connection()?;
        let job = Job::from(job);
        let job = diesel::insert_into(jobs::table)
            .values(job)
            .get_result(&conn)?;
        Ok(job)
    }
}

impl Job {
    fn from(job: Job) -> Job {
        Job {
            title: job.title,
            description: job.description,
            salary: job.salary,
            job_type: job.job_type,
            is_remote: job.is_remote,
            currency: job.currency,
            apply_url: job.apply_url,
            job_city: job.job_city,
            job_email: job.job_email,
            company: job.company,
            company_twitter: job.company_twitter,
            company_website: job.company_website,
            company_logo: job.company_logo,
            slug: job.slug,
            created_at: job.created_at,
            updated_at: job.updated_at
        }
    }
}
