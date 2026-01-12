mod build_key_value_table;
mod build_environment_variable_table;
mod build_instance;
mod shared;

use rusqlite::Connection;
use sha1::digest::crypto_common::Key;
use crate::breakpoint_trap_result;
use crate::database::build_key_value_table::KeyValue;

pub enum InitializeDatabaseResult {

}

pub fn initialize_database(database_path: String) -> Option<String> {
    let rusqlite_connection = match Connection::open(&database_path) {
        Ok(x) => { x }
        Err(_) => {
            panic!()
        }
    };

    let user_settings = KeyValue::init(&rusqlite_connection, "user_settings").unwrap();


    panic!()
}

pub fn load_database(connection: &mut String) {

}

pub fn new_database(connection: &mut String) {

}