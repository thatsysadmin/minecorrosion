use std::collections::HashMap;
use std::ops::Index;
use rusqlite::Connection;
use crate::database::build_instance::BuildInstanceResult::{KeyPrefixStripFailed, KeyValueRecallFailed, OK};
use crate::database::build_key_value_table::{key_value_get_all_keys, key_value_init, key_value_set_key, KeyValue, KeyValueResult};

pub struct MCInstance {
    pub table_version: Option<String>,
    pub game_version: Option<String>,
    pub game_directory: Option<String>,
    pub assets_root: Option<String>,
    pub assets_index_name: Option<String>,
    pub version_type: Option<String>,
    pub resolution_width: Option<String>,
    pub resolution_height: Option<String>,
    pub main_class: Option<String>,
    pub max_heap: Option<String>,
    pub min_heap: Option<String>,
    pub cmdline_arguments: Option<String>,
}

pub enum MCInstanceKeyIdentifier {
    table_version,
    game_version,
    game_directory,
    assets_root,
    assets_index_name,
    version_type,
    resolution_width,
    resolution_height,
    main_class,
    max_heap,
    min_heap,
    cmdline_arguments,
}

impl MCInstanceKeyIdentifier {
    pub fn get_key_name(&self) -> &str {
        match self {
            MCInstanceKeyIdentifier::table_version => {"table_version"}
            MCInstanceKeyIdentifier::game_version => {"game_version"}
            MCInstanceKeyIdentifier::game_directory => {"game_directory"}
            MCInstanceKeyIdentifier::assets_root => {"assets_root"}
            MCInstanceKeyIdentifier::assets_index_name => {"assets_index_name"}
            MCInstanceKeyIdentifier::version_type => {"version_type"}
            MCInstanceKeyIdentifier::resolution_width => {"resolution_width"}
            MCInstanceKeyIdentifier::resolution_height => {"resolution_height"}
            MCInstanceKeyIdentifier::main_class => {"main_class"}
            MCInstanceKeyIdentifier::max_heap => {"max_heap"}
            MCInstanceKeyIdentifier::min_heap => {"min_heap"}
            MCInstanceKeyIdentifier::cmdline_arguments => {"cmdline_arguments"}
        }
    }
}

fn process_mc_instance_elements<CB>(mcinstance: MCInstance, callback: CB) where CB: Fn(Option<String>, MCInstanceKeyIdentifier) {
    callback(mcinstance.table_version, MCInstanceKeyIdentifier::table_version);
    callback(mcinstance.game_version, MCInstanceKeyIdentifier::game_version);
    callback(mcinstance.game_directory, MCInstanceKeyIdentifier::game_directory);
    callback(mcinstance.assets_root, MCInstanceKeyIdentifier::assets_root);
    callback(mcinstance.assets_index_name, MCInstanceKeyIdentifier::assets_index_name);
    callback(mcinstance.version_type, MCInstanceKeyIdentifier::version_type);
    callback(mcinstance.resolution_width, MCInstanceKeyIdentifier::resolution_width);
    callback(mcinstance.resolution_height, MCInstanceKeyIdentifier::resolution_height);
    callback(mcinstance.main_class, MCInstanceKeyIdentifier::main_class);
    callback(mcinstance.max_heap, MCInstanceKeyIdentifier::max_heap);
    callback(mcinstance.min_heap, MCInstanceKeyIdentifier::min_heap);
    callback(mcinstance.cmdline_arguments, MCInstanceKeyIdentifier::cmdline_arguments);
}

/// All parameters of Instance are required.
pub fn build_instance_create_instance(database: &Connection, instance_name: String, initial_mcinstance: MCInstance) -> BuildInstanceResult<()> {
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
    // for (index, bare_key) in keys.iter().enumerate() {
    //     let key = format!("variable:{}", bare_key);
    //
    //
    //
    //     match key_value_set_key(database, &table_name, &key, None) {
    //         KeyValueResult::OK(_) => {}
    //         KeyValueResult::KeyReplaced => {}
    //         KeyValueResult::NewKeyCreated => {}
    //         (error) => {
    //             panic!()
    //         }
    //     };
    // }

    process_mc_instance_elements(initial_mcinstance, |instance_x, key_identifier| {
        let key_name = key_identifier.get_key_name();
        let instance = match instance_x {
            Some(element) => {
                match key_value_set_key(database, &table_name, key_name, Some(element.as_str())) {
                    KeyValueResult::OK(_) => {}
                    KeyValueResult::KeyReplaced => {}
                    KeyValueResult::NewKeyCreated => {}
                    (error) => {
                        panic!()
                    }
                }
            }
            None => {
                match key_value_set_key(database, &table_name, key_name, None) {
                    KeyValueResult::OK(_) => {}
                    KeyValueResult::KeyReplaced => {}
                    KeyValueResult::NewKeyCreated => {}
                    (error) => {
                        panic!()
                    }
                }
            }
        };
    });

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

pub fn build_instance_get_variables(database: &Connection, instance_name: String) {

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