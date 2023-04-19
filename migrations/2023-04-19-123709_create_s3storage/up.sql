-- Your SQL goes here

CREATE Table s3storage (
    id SERIAL PRIMARY KEY,
    bucket_name VARCHAR(64),
    s3_key VARCHAR(128)
)