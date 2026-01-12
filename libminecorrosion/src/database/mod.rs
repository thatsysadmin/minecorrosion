mod build_key_value_table;
mod build_environment_variable_table;
mod build_instance;

use crate::breakpoint_trap_result;

pub enum InitializeDatabaseResult {

}

pub fn initialize_database(database_path: String) -> Option<String> {


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

pub fn load_database(connection: &mut String) {

}

pub fn new_database(connection: &mut String) {
    
}