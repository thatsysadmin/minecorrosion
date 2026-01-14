use std::fmt::format;
use std::os::macos::raw::stat;
use rusqlite::{params, Connection, Statement};
use rusqlite::fallible_streaming_iterator::FallibleStreamingIterator;
use crate::database::build_key_value_table::KeyValueResult::{NoRowsFound, PreperationFailed, QueryFailed, OK};
use crate::database::shared::sanitize;

pub struct KeyValue {
    key: String,
    value: String,
}

pub struct KeyValueContainer<'a> {
    database: &'a Connection,
    table_name: String,
}

impl<'a> KeyValueContainer<'a> {
    pub fn init(database: &'a Connection, table_name: &'a str) -> Result<Self, KeyValueResult<()>> {
        let table_name_internal = format!("kv:{}", table_name);
        let mut table_query = match database.prepare("SELECT EXISTS (SELECT name FROM sqlite_master WHERE type='table' AND name=?1)") {
            Ok(x) => { x }
            Err(_) => {
                panic!()
            }
        };

        let table_exists = match table_query.query_row(params![table_name_internal], |row| row.get::<usize, bool>(0)) {
            Ok(x) => { x }
            Err(_) => {
                panic!()
            }
        };
        if !table_exists {
            println!("New database, building out.");
            let mut build_query = match database.prepare(&format!("CREATE TABLE \'{}\' (key TEXT, value TEXT)", sanitize(&table_name_internal))) {
                Ok(x) => { x }
                Err(_) => {
                    panic!()
                }
            };
            match build_query.execute(()) {
                Ok(_) => {}
                Err(_) => {
                    panic!()
                }
            };
        }
        else {
            println!("Database already created.");
        }

        Ok(KeyValueContainer {
            database,
            table_name: table_name_internal,
        })
    }

    pub fn get_key(&self, target_key: &str) -> KeyValueResult<String> {
        let mut statement = match self.database.prepare(&format!("SELECT value FROM \'{}\' WHERE key=?1", self.table_name)) {
            Ok(x) => x,
            Err(e) => {
                return PreperationFailed
            },
        };
        let result = match statement.query_row(params![target_key], |row| row.get::<usize, String>(0)) {
            Ok(x) => x,
            Err(e) => {
                match e {
                    rusqlite::Error::QueryReturnedNoRows => {
                        return NoRowsFound
                    }
                    _ => {
                        return QueryFailed
                    }
                }
            }
        };
        OK(result)
    }

    pub fn get_all_keys(&self) -> KeyValueResult<Vec<KeyValue>> {
        let mut statement = match self.database.prepare(&format!("SELECT * FROM \'{}\'", self.table_name)) {
            Ok(x) => { x }
            Err(e) => {
                return PreperationFailed
            }
        };
        let statement = match statement.query_map([], |row| {
            Ok(KeyValue {
                key: row.get(0).unwrap(),
                value: row.get(1).unwrap()
            })
        }) {
            Ok(x) => x,
            Err(e) => {
                return QueryFailed
            }
        };
        let mut x: Vec<KeyValue> = Vec::new();
        for y in statement {
            x.push(y.unwrap());
        };
        OK(x)
    }

    pub fn set_key(&self, key: &str, value: &str) -> KeyValueResult<String> {
        let table_name = &self.table_name;
        let database = self.database;
        let key_exists: bool;
        match &self.get_key(key) {
            OK(_) => {
                key_exists = true;
            }
            e => {
                match e {
                    NoRowsFound => {
                        key_exists = false;
                    }
                    _ => {
                        return QueryFailed
                    }
                }
            }
        };
        let sql_call = if !key_exists { // Add a key
            format!("INSERT INTO \'{}\' (key, value) VALUES (?2, ?1)", table_name)
        }
        else { // Replace a key
            format!("UPDATE \'{}\' SET value=?1 WHERE key=?2", table_name)
        };

        let mut statement = match database.prepare(&sql_call) {
            Ok(x) => { x }
            Err(_) => {
                panic!()
            }
        };

        let result = match statement.execute(params![value, key]) {
            Ok(x) => {x}
            Err(e) => {
                panic!()
            }
        };

        panic!()
    }

    pub fn delete_key(self, key: &str) {

    }
}

#[derive(Debug)]
pub enum KeyValueResult<T> {
    OK(T),
    KeyReplaced,
    NewKeyCreated,
    PreperationFailed,
    QueryFailed,
    NoRowsFound,
    UnknownFailure
}