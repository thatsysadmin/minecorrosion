use diesel::{Insertable, Queryable, Selectable};
use crate::schema::*;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = minecorrosion_settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MineCorrosionSettings {
    pub id: i32,
    pub database_version: i32,
}