// @generated automatically by Diesel CLI.

diesel::table! {
    mavens (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        group_id -> Nullable<Varchar>,
        artifact_id -> Nullable<Varchar>,
        packaging -> Nullable<Varchar>,
        version -> Nullable<Varchar>,
        bucket_name -> Nullable<Varchar>,
        path -> Nullable<Varchar>,
        cloud -> Nullable<Bool>,
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
