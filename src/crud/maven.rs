use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use crate::models::maven::NewMaven;
use diesel::result::Error;
use crate::schema::mavens;
pub fn insert_maven(
    conn: &mut PgConnection, 
    name: &str,
    group_id: &str,
    artifact_id: &str,
    packaging: &str,
    version: &str,
    bucket_name: &str,
    path: &str,
    cloud: &bool) -> Result<usize, Error>{

    let new_maven = NewMaven{name, group_id,artifact_id, packaging, version, bucket_name, path, cloud};
    diesel::insert_into(mavens::table)
        .values(&new_maven)
        .execute(conn)
}

pub fn delete_maven(
    conn: &mut PgConnection,
    name: &str,
    group_id: &str,
    artifact_id: &str,
    packaging: &str,
    version: &str,
) -> Result<usize, Error> {
    diesel::delete(mavens::table.filter(
    mavens::name.eq(name)
    .and(mavens::group_id.eq(group_id)
    .and(mavens::artifact_id.eq(artifact_id)
    .and(mavens::packaging.eq(packaging)
    .and(mavens::version.eq(version)))))))
    .execute(conn)
}