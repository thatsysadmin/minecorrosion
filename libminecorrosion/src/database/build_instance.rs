

pub struct Instance {

}

impl Instance {
    pub fn create_instance(instance_name_z: &String) -> Self {
        let instance_name = format!("instance:{}", instance_name_z);


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

        }

        panic!()
    }

    pub fn get_instance(instance_name: &String) {

    }

    pub fn modify_instance(instance_name: &String) {

    }

    pub fn delete_instance(instance_name: &String) {

    }
}