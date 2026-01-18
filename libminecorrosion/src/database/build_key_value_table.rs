use std::fmt::format;
use std::os::macos::raw::stat;
use rusqlite::{params, Connection, Error, Statement};
use rusqlite::fallible_streaming_iterator::FallibleStreamingIterator;
use rusqlite::ffi::SQLITE_NULL;
use crate::database::build_key_value_table::KeyValueResult::{KeyReplaced, NewKeyCreated, NoRowsFound, PreperationFailed, QueryFailed, RowQueryFailure, RowQueryFailureKey, RowQueryFailureValue, UnknownFailure, OK};
use crate::database::shared::sanitize;

pub struct KeyValue {
    pub key: String,
    pub value: String,
}

pub struct KeyValueRusqliteResult {
    pub key: rusqlite::Result<String>,
    pub value: rusqlite::Result<String>,
}

pub fn key_value_init(database: &Connection, table_name: &str) -> KeyValueResult<()> {
    let mut table_query = match database.prepare("SELECT EXISTS (SELECT name FROM sqlite_master WHERE type='table' AND name=?1)") {
        Ok(x) => { x }
        Err(_) => {
            return PreperationFailed
        }
    };

    let table_exists = match table_query.query_row(params![table_name], |row| row.get::<usize, bool>(0)) {
        Ok(x) => { x }
        Err(_) => {
            return QueryFailed
        }
    };
    if !table_exists {
        println!("New database, building out.");
        let mut build_query = match database.prepare(&format!("CREATE TABLE \'{}\' (key TEXT, value TEXT)", sanitize(table_name))) {
            Ok(x) => { x }
            Err(_) => {
                return PreperationFailed
            }
        };
        match build_query.execute(()) {
            Ok(_) => {}
            Err(_) => {
                return QueryFailed
            }
        };
    }
    else {
        println!("Database already created.");
    }

    OK(())
}

pub fn key_value_get_key(database: &Connection, table_name: &str, target_key: &str) -> KeyValueResult<String> {
    let mut statement = match database.prepare(&format!("SELECT value FROM \'{}\' WHERE key=?1", table_name)) {
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

pub fn key_value_get_all_keys(database: &Connection, table_name: &str, search_query: Option<&str>) -> KeyValueResult<Vec<KeyValue>> {
    let mut statement = match database.prepare(&format!("SELECT * FROM \'{}\'", table_name)) {
        Ok(x) => { x }
        Err(e) => {
            return PreperationFailed
        }
    };
    let statement = match statement.query_map([], |row| {
        Ok(
            KeyValueRusqliteResult {
                key: row.get(0),
                value: row.get(1),
            }
        )
    }) {
        Ok(x) => x,
        Err(e) => {
            return QueryFailed
        }
    };
    let mut x: Vec<KeyValue> = Vec::new();
    let mut row_index: usize = 0;
    for (row_index, y) in statement.enumerate() {
        x.push(
            match y {
                Ok(x) => {
                    let key = match x.key {
                        Ok(x) => { x }
                        Err(e) => {
                            return RowQueryFailureKey(row_index)
                        }
                    };
                    let value = match x.value {
                        Ok(x) => { x }
                        Err(e) => {
                            match e {
                                Error::InvalidColumnType(_, _, sqlite_type) => {
                                    if sqlite_type == rusqlite::types::Type::Null {
                                        "$NOTATHING".to_string()
                                    }
                                    else {
                                        return RowQueryFailureValue(row_index)
                                    }
                                }
                                _ => {
                                    return RowQueryFailureKey(row_index)
                                }
                            }
                        }
                    };
                    KeyValue { key, value }
                },
                Err(e) => {
                    return RowQueryFailure(row_index)
                }
            }
        );
        // row_index += 1;
    };
    OK(x)
}

pub fn key_value_set_key(database: &Connection, table_name: &str, key: &str, value: Option<&str>) -> KeyValueResult<()> {
    let table_name = &table_name;
    let database = database;
    let key_exists: bool;
    match key_value_get_key(database, table_name, key) {
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
            return PreperationFailed
        }
    };

    let params = match value {
        None => { params![None::<String>, key] }
        Some(x) => { params![x.to_string(), key] }
    };
    let result = match statement.execute(params) {
        Ok(x) => {x}
        Err(e) => {
            return QueryFailed
        }
    };

    if result == 0 {
        QueryFailed
    }
    else if result == 1 {
        if key_exists {
            KeyReplaced
        }
        else {
            NewKeyCreated
        }
    }
    else {
        UnknownFailure
    }
}

pub fn key_value_delete_key(database: &Connection, table_name: &str, key: &str) -> KeyValueResult<()> {
    let mut statement = match database.prepare(&format!("DELETE FROM \'{}\' WHERE key=?1", table_name)) {
        Ok(x) => x,
        Err(e) => {
            return PreperationFailed
        }
    };
    let result = match statement.execute(params![key]) {
        Ok(x) => { x }
        Err(e) => {
            return QueryFailed
        }
    };
    if result == 0 {
        QueryFailed
    }
    else if result == 1 {
        OK(())
    }
    else {
        UnknownFailure
    }
}

#[derive(Debug)]
pub enum KeyValueResult<T> {
    OK(T),
    KeyReplaced, // Only applies to set_key()
    NewKeyCreated, // Only applies to set_key()
    PreperationFailed,
    QueryFailed,
    NoRowsFound,
    RowQueryFailure(usize),
    RowQueryFailureKey(usize),
    RowQueryFailureValue(usize),
    UnknownFailure
}