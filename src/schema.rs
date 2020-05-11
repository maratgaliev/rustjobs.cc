table! {
    jobs (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        salary -> Nullable<Int4>,
        currency -> Varchar,
        apply_url -> Nullable<Varchar>,
        job_city -> Varchar,
        job_email -> Varchar,
        company -> Varchar,
        company_twitter -> Nullable<Varchar>,
        company_website -> Varchar,
        company_logo -> Nullable<Varchar>,
        slug -> Varchar,
    }
}
