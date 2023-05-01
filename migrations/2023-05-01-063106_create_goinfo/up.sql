-- Your SQL goes here

CREATE TABLE Goinfos (
    id SERIAL PRIMARY KEY,
    package VARCHAR NOT NULL,
    owner VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    time VARCHAR NOT NULL,
    domain VARCHAR NOT NULL,
    bucket_name VARCHAR NOT NULL,
    path VARCHAR NOT NULL,
    cloud BOOLEAN NOT NULL
)