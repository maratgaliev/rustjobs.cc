table! {
    jobs (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        salary -> Nullable<Int4>,
        job_type -> Varchar,
        is_remote -> Nullable<Bool>,
        currency -> Varchar,
        apply_url -> Nullable<Varchar>,
        job_city -> Varchar,
        job_email -> Nullable<Varchar>,
        company -> Varchar,
        company_twitter -> Nullable<Varchar>,
        company_website -> Varchar,
        company_logo -> Nullable<Varchar>,
        slug -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
