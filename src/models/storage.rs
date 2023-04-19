#[allow(dead_code)]
pub struct S3Storage {
    pub bucket_name: String,
    pub s3key: String,
}

#[allow(dead_code)]
pub struct LocalStorage {
    pub path: String,
}