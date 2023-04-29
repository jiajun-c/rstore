use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use crate::models::maven::{NewMaven, Maven};
use diesel::result::Error;
use crate::schema::mavens::dsl::*;

pub fn insert_maven(
    conn: &mut PgConnection, 
    m_name: &str,
    m_group_id: &str,
    m_artifact_id: &str,
    m_packaging: &str,
    m_version: &str,
    m_bucket_name: &str,
    m_path: &str,
    m_cloud: &bool) -> Result<usize, Error>{
    let new_maven = NewMaven{
        name: m_name,
        group_id: m_group_id,
        artifact_id: m_artifact_id,
        packaging: m_packaging,
        version: m_version,
        bucket_name: m_bucket_name,
        path: m_path,
        cloud: m_cloud};
    diesel::insert_into(mavens)
        .values(&new_maven)
        .execute(conn)
}

pub fn delete_maven(
    conn: &mut PgConnection,
    m_name: &str,
    m_group_id: &str,
    m_artifact_id: &str,
    m_packaging: &str,
    m_version: &str,
) -> Result<usize, Error> {
    diesel::delete(mavens.filter(
    name.eq(m_name)
    .and(group_id.eq(m_group_id))
    .and(artifact_id.eq(m_artifact_id))
    .and(packaging.eq(m_packaging))
    .and(version.eq(m_version))))
    .execute(conn)
}

pub fn get_maven(
    conn: &mut PgConnection,
    m_name: &str,
    m_group_id: &str,
    m_artifact_id: &str,
    m_packaging: &str,
    m_version: &str,
) -> (bool, String) {
    let ans = mavens.filter(name.eq(m_name)
        .and(group_id.eq(m_group_id))
        .and(artifact_id.eq(m_artifact_id))
        .and(version.eq(m_version))
        .and(packaging.eq(m_packaging)))
        .limit(1)
        .load::<Maven>(conn)
    .expect("failed to get conn");
    if ans.len() == 0 {
        (false, String::from(""))
    } else {
        (ans[0].cloud, ans[0].path.clone())
    }
}