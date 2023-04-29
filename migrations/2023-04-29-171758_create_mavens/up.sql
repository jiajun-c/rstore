-- Your SQL goes here-- Your SQL goes here
CREATE Table mavens (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    group_id VARCHAR NOT NULL, 
    artifact_id VARCHAR NOT NULL,
    packaging VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    bucket_name VARCHAR NOT NULL,
    path VARCHAR NOT NULL,
    cloud BOOLEAN NOT NULL
)