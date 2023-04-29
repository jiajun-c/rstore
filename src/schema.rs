// @generated automatically by Diesel CLI.

diesel::table! {
    mavens (id) {
        id -> Int4,
        name -> Varchar,
        group_id -> Varchar,
        artifact_id -> Varchar,
        packaging -> Varchar,
        version -> Varchar,
        bucket_name -> Varchar,
        path -> Varchar,
        cloud -> Bool,
    }
}

diesel::table! {
    s3storage (id) {
        id -> Int4,
        bucket_name -> Nullable<Varchar>,
        s3_key -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    mavens,
    s3storage,
);
