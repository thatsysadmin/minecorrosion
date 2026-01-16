use rusqlite::Connection;
use crate::database::build_key_value_table::{KeyValueContainer, KeyValueResult};

pub struct Instance<'a> {
    keyvalue_store: KeyValueContainer<'a>
}

impl Instance<'_> {
    pub fn create_instance(database_connection: &Connection, instance_name_z: String) -> Self {
        let instance_name = format!("instance:{}", instance_name_z);
        let kv_store = match KeyValueContainer::init(database_connection, &instance_name) {
            Ok(x) => { x }
            Err(e) => {
                panic!()
            }
        };

        let keys = [
            "table_version",
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
        for key in keys {
            match kv_store.set_key(key, None) {
                KeyValueResult::OK(_) => {},
                KeyValueResult::KeyReplaced => {},
                KeyValueResult::NewKeyCreated => {},
                _ => {
                    panic!()
                }
            };
        }

        Instance {
            keyvalue_store: kv_store
        }
    }

    pub fn get_instance(&self) {

    }

    pub fn modify_instance(&self, instance_name: &String) {

    }

    pub fn delete_instance(&self, instance_name: &String) {

    }

    fn build_instance_in_kv(&self, instance_name: &String) {

    }
}