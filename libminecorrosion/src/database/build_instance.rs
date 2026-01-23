use std::collections::HashMap;
use std::ops::Index;
use rusqlite::Connection;
use crate::database::build_instance::BuildInstanceResult::{KeyPrefixStripFailed, KeyValueRecallFailed, OK};
use crate::database::build_key_value_table::{key_value_get_all_keys, key_value_init, key_value_set_key, KeyValue, KeyValueResult};

pub struct MCInstance {
    table_version: Option<String>,
    game_version: Option<String>,
    game_directory: Option<String>,
    assets_root: Option<String>,
    assets_index_name: Option<String>,
    version_type: Option<String>,
    resolution_width: Option<String>,
    resolution_height: Option<String>,
    main_class: Option<String>,
    max_heap: Option<String>,
    min_heap: Option<String>,
    cmdline_arguments: Option<String>,
}

impl<T> Index<usize> for MCInstance<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        match index {
            n => panic!("index {} is out of bounds", n),
            
        }
    }
}

/// All parameters of Instance are required.
pub fn build_instance_create_instance(database: &Connection, instance_name: String, instance: MCInstance) -> BuildInstanceResult<()> {
    let table_name = format!("kv:mcinstance:{}", instance_name);
    match key_value_init(database, &table_name) {
        KeyValueResult::OK(_) => {}
        _ => {
            panic!()
        }
    };
    let keys = [
        //
        // What's not included - should be handled dynamically
        // - Username
        // - Mojang UUID?
        // - Xbox UUID?
        // - AccessToken
        // - Assets root, unless redirected
        // - game_directory, unless redirected
        //
        // - Launcher brand
        // - Launcher version
        //
        // Probably don't implement quickplay functionality.
        //
        "table_version",
        "game_version",
        "game_directory",
        "assets_root",
        "assets_index_name",
        "version_type",
        "resolution_width",
        "resolution_height",
        "main_class",
        "max_heap",
        "min_heap",
        "cmdline_arguments",
    ];
    for (index, bare_key) in keys.iter().enumerate() {
        let key = format!("variable:{}", bare_key);
        match key_value_set_key(database, &table_name, &key, None) {
            KeyValueResult::OK(_) => {}
            KeyValueResult::KeyReplaced => {}
            KeyValueResult::NewKeyCreated => {}
            (error) => {
                panic!()
            }
        };
    }

    OK(())
}

pub fn build_instance_get_instance<'a>(database: &Connection, instance_name: String) -> BuildInstanceResult<HashMap<String, String>> {
    let table_name = format!("kv:mcinstance:{}", instance_name);
    let kvs = match key_value_get_all_keys(database, &table_name, None) {
        KeyValueResult::OK(x) => { x }
        (error_result) => {
            return KeyValueRecallFailed
        }
    };

    let mut environment_variables: HashMap<String, String> = HashMap::new();
    for kv in kvs {
        let key = match kv.key.strip_prefix("variable:") {
            None => {
                return KeyPrefixStripFailed
            }
            Some(x) => { x }
        };
        environment_variables.insert(key.to_string(), kv.value);
    }

    OK(environment_variables) // This would output the environment variables and possibly the rules to feed into the parsers
}

pub fn build_instance_modify_instance(database: &Connection, instance_name: String) -> BuildInstanceResult<()> {
    let table_name = format!("kv:mcinstance:{}", instance_name);

    panic!()
}

pub fn build_instance_delete_instance(database: &Connection, instance_name: String) -> BuildInstanceResult<()> {
    let table_name = format!("kv:mcinstance:{}", instance_name);

    panic!()
}

pub fn build_instance_in_kv(database: &Connection, instance_name: String) -> BuildInstanceResult<()> {
    let table_name = format!("kv:mcinstance:{}", instance_name);

    panic!()
}

#[derive(Debug)]
pub enum BuildInstanceResult<T> {
    OK(T),
    KeyValueRecallFailed,
    KeyPrefixStripFailed,
    UnknownFailure
}