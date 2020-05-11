CREATE TABLE jobs (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  salary INT,
  currency character varying(255) NOT NULL,
  apply_url character varying(255),
  job_city character varying(255) NOT NULL,
  job_email character varying(255) NOT NULL,
  company character varying(255) NOT NULL,
  company_twitter character varying(255),
  company_website character varying(255) NOT NULL,
  company_logo character varying(255),
  slug character varying(255) NOT NULL
);