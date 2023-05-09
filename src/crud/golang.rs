use core::time;

use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods, BoolExpressionMethods};

use crate::models::golang::{NewGoModule, GoModule, GoInfo};
use crate::schema::{gomodule::dsl::*, goinfo::dsl::*};
use diesel::result::Error;

pub fn insert_go_module(
    conn: &mut PgConnection,
    go_base: &str,
    go_module: &str
) -> Result<usize, Error>{
    let new_go_module = NewGoModule{
        base: go_base,
        module: go_module,
    };
    diesel::insert_into(gomodule)
        .values(&new_go_module)
        .execute(conn)
}

pub fn get_go_module_id(
    conn: &mut PgConnection,
    go_base: &str,
    go_module: &str
) -> Result<i32, Error> {
    let ans = gomodule.filter(base.eq(go_base)
        .and(module.eq(go_module)))
        .limit(1)
        .load::<GoModule>(conn)
        .expect("failed to get go module");
    if ans.len() == 0 {
        Err(Error::NotFound)
    } else {
        Ok(ans[0].id)
    }
}

pub fn delete_go_module(
    conn: &mut PgConnection,
    go_base: &str,
    go_module: &str
) -> Result<usize, Error> {
    diesel::delete(gomodule.filter(
        base.eq(go_base)
        .and(module.eq(go_module))))
        .execute(conn)
}

pub fn insert_go_info(
    conn: &mut PgConnection,
    go_id: i32,
    go_version: &str,
    go_time: &str,
    go_path: &str
) -> Result<usize, Error> {
    let new_go_info = GoInfo{
        id: go_id,
        version: go_version.to_string(),
        time: go_time.to_string(),
        path: go_path.to_string()
    };
    diesel::insert_into(goinfo)
        .values(&new_go_info)
        .execute(conn)
}