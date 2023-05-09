// @generated automatically by Diesel CLI.

diesel::table! {
    goinfo (id) {
        id -> Int4,
        version -> Nullable<Varchar>,
        time -> Nullable<Varchar>,
        path -> Nullable<Varchar>,
    }
}

diesel::table! {
    gomodule (id) {
        id -> Int4,
        base -> Varchar,
        module -> Varchar,
    }
}

diesel::table! {
    govesioninfo (mid) {
        mid -> Int4,
        vsc -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        go_ref -> Nullable<Varchar>,
        go_path -> Nullable<Varchar>,
    }
}

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
    goinfo,
    gomodule,
    govesioninfo,
    mavens,
    s3storage,
);
