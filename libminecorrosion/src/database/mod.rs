use std::path::{Path, PathBuf};
use diesel::prelude::*;
use diesel::{declare_sql_function, Connection, ConnectionResult, SqliteConnection};
use crate::breakpoint_trap_result;
use crate::models::MineCorrosionSettings;
use crate::schema::minecorrosion_settings::database_version;
use crate::schema::minecorrosion_settings::dsl::minecorrosion_settings;

pub fn initialize_database(database_path: String) -> Option<String> {
    let mut connection = breakpoint_trap_result(SqliteConnection::establish(&database_path)).unwrap();

    diesel::insert_into(minecorrosion_settings)
        .values(MineCorrosionSettings {
            id: 0,
            database_version: 0,
        })
        .returning(MineCorrosionSettings::as_returning())
        .get_result(&mut connection)
        .unwrap();

    let db_version = minecorrosion_settings
        .select(MineCorrosionSettings::as_select())
        .load(&mut connection)
        .unwrap();


    panic!()
}

pub fn load_database(connection: &mut SqliteConnection) {

}

pub fn new_database(connection: &mut SqliteConnection) {
    // These are probably safe.
    diesel::sql_query("CREATE TABLE minecorrosion_settings (id INTEGER, database_version INTEGER)").execute(connection).unwrap();
    
}