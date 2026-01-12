use rusqlite::{params, Connection};
use rusqlite::fallible_streaming_iterator::FallibleStreamingIterator;
use crate::database::shared::sanitize;

pub struct KeyValue<'a> {
    database: &'a Connection,
    table_name: String,
}

impl<'a> KeyValue<'a> {
    pub fn init(database: &'a Connection, table_name: &'a str) -> Result<Self, KeyValueErrors> {
        let table_name_internal = format!("kv:{}", table_name);
        let mut table_query = database.prepare(&format!("SELECT EXISTS (SELECT name FROM sqlite_master WHERE type='table' AND name=?1)")).unwrap();
        let x = table_query.query_row(params![table_name_internal], |row| row.get::<usize, bool>(0));

        panic!();
        let creation_result = database.execute("CREATE TABLE (?1) (
            key TEXT,
            value TEXT,
        )", params![table_name_internal]).unwrap();

        Ok(KeyValue {
            database,
            table_name: table_name_internal,
        })
    }

    pub fn get_key(self) {

    }

    pub fn get_all_keys(self) {

    }

    pub fn set_key(self) {

    }

    pub fn delete_key(self) {

    }
}

#[derive(Debug)]
pub enum KeyValueErrors {

}