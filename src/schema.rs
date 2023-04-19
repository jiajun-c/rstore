// @generated automatically by Diesel CLI.

diesel::table! {
    s3storage (id) {
        id -> Int4,
        bucket_name -> Nullable<Varchar>,
        s3_key -> Nullable<Varchar>,
    }
}
