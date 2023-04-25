-- Your SQL goes here-- Your SQL goes here
CREATE Table mavens (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128),
    group_id VARCHAR(128), 
    artifact_id VARCHAR(128),
    packaging VARCHAR(128),
    version VARCHAR(128),
    bucket_name VARCHAR(128),
    path VARCHAR(256),
    cloud BOOLEAN
)