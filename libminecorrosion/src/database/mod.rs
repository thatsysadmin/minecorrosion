mod build_table;

use std::path::{Path, PathBuf};
use diesel::prelude::*;
use diesel::{declare_sql_function, Connection, ConnectionResult, SqliteConnection};
use diesel::connection::SimpleConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::breakpoint_trap_result;
use crate::models::MineCorrosionSettings;
use crate::schema::minecorrosion_settings::dsl::minecorrosion_settings;
use crate::schema::minecorrosion_settings::{key, value};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub enum InitializeDatabaseResult {

}

pub fn initialize_database(database_path: String) -> Option<String> {
    let mut connection = breakpoint_trap_result(SqliteConnection::establish(&database_path)).unwrap();

    // create initial tables by migrating
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(t) => {

        }
        Err(e) => {
            return None
        }
    };

    let res1lt = match minecorrosion_settings
        .select(MineCorrosionSettings::as_select())
        .filter(value.eq("doesntexist"))
        .load(&mut connection) {
        Ok(x) => { x }
        Err(_) => {
            panic!()
        }
    };

    println!("break");
    // diesel::insert_into(minecorrosion_settings)
    //     .values(MineCorrosionSettings {
    //         key: "".to_string(),
    //         value: "".to_string()
    //     })
    //     .returning(MineCorrosionSettings::as_returning())
    //     .get_result(&mut connection)
    //     .unwrap();

    // let db_version = minecorrosion_settings
    //     .select(MineCorrosionSettings::as_select())
    //     .load(&mut connection)
    //     .unwrap();


    panic!()
}

pub fn load_database(connection: &mut SqliteConnection) {

}

pub fn new_database(connection: &mut SqliteConnection) {
    // These are probably safe.
    diesel::sql_query("CREATE TABLE minecorrosion_settings (id INTEGER, database_version INTEGER)").execute(connection).unwrap();

}