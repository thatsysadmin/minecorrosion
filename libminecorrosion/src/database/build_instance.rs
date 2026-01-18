use std::collections::HashMap;
use rusqlite::Connection;
use crate::database::build_instance::BuildInstanceResult::OK;
use crate::database::build_key_value_table::{key_value_get_all_keys, key_value_init, key_value_set_key, KeyValue, KeyValueResult};

pub struct Instance {
    table_version: String,
    game_version: String,
    game_directory: String,
    assets_root: String,
    assets_index_name: String,
    version_type: String,
    resolution_width: String,
    resolution_height: String,
    main_class: String,
    max_heap: String,
    min_heap: String,
    cmdline_arguments: String,
}

pub fn build_instance_create_instance(database: &Connection, instance_name: String) -> BuildInstanceResult<()> {
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
    for bare_key in keys {
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
            panic!()
        }
    };

    let mut environment_variables: HashMap<String, String> = HashMap::new();
    for kv in kvs {
        let key = match kv.key.strip_prefix("variable:") {
            None => {
                panic!()
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

pub fn build_instance_build_instance_in_kv(database: &Connection, instance_name: String) -> BuildInstanceResult<()> {
    let table_name = format!("kv:mcinstance:{}", instance_name);

    panic!()
}

#[derive(Debug)]
pub enum BuildInstanceResult<T> {
    OK(T),

    UnknownFailure
}