use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;
use crate::configure::assets::{download_assets, get_assets};
use crate::configure::game_slash_jvm::parse_arguments_game_plus_jvm;
use crate::configure::libraries::{get_libraries, ArtifactStructure};

mod shared;
mod game_slash_jvm;
mod libraries;
mod assets;

pub fn store_configuration(configuration_path: &std::path::Path) {
    
}

pub fn build_cmdline() {
    // TODO: Temporary
    let os = std::env::consts::OS;

    let minecraft_json = "/Volumes/tmpfs/minecraft_launch_poc/reference/1.21.10_pretty.json";
    let file_contents = fs::read(Path::new(minecraft_json)).unwrap();
    let file_str = str::from_utf8(&*file_contents).unwrap();
    let test_config = serde_json::from_str::<Value>(file_str).unwrap();

    let mut test_rules: HashMap<&str, bool> = HashMap::new();
    test_rules.insert("testtrue", true);
    test_rules.insert("testfalse", false);
    test_rules.insert("has_custom_resolution", true);

    let library_root = Path::new("/Volumes/tmpfs/minecorrosion_libraries/jar");
    fs::create_dir_all(library_root).unwrap();
    let jni_root = Path::new("/Volumes/tmpfs/minecorrosion_libraries/");
    fs::create_dir_all(jni_root).unwrap();

    let mut environment_variables: HashMap<&str, &str> = HashMap::new();
    let mut rules: HashMap<&str, bool> = HashMap::new();

    let client = reqwest::blocking::Client::new();

    // TODO: ---------------------------------------------------------------------

    // TODO: Native JNI stuff
    environment_variables.insert("natives_directory", library_root.to_str().unwrap());

    // TODO: Libraries
    let libraries_json = test_config.get("libraries").unwrap();
    let libraries = get_libraries(libraries_json, os, test_rules).unwrap();
    crate::configure::libraries::download_libraries(&client, libraries.clone(), "/Volumes/tmpfs/minecorrosion_libraries/jar");
    let mut library_arguments = vec!["-cp ".to_string()];
    let mut library_incomplete = false;
    let mut library_index: usize = 0;
    let libraries_length = libraries.len();
    for library in libraries {
        let path = Path::new(&library.path);
        if !(path.exists() || path.is_file()) {
            library_incomplete = true;
            break;
        }
        // TODO: Hash the file with the strongest type available
        let merge = library_root.to_owned().join(path);
        if library_index == libraries_length - 1 {
            library_arguments.push(merge.to_str().unwrap().to_string());
        }
        else {
            library_arguments.push(format!("{}:", merge.to_str().unwrap().to_string()))
        }
        library_index += 1;
    }
    // if library_incomplete {
    //
    // }
    // else {
    //
    // }

    // TODO: Get assets
    let assets_path = Path::new("/Users/h/Library/Application Support/minecraft/assets/indexes/27.json");
    let assets_file_handle = fs::read(assets_path).unwrap();
    let assets_str = str::from_utf8(&*assets_file_handle).unwrap();
    let assets_json: Value = serde_json::from_str(assets_str).unwrap();
    let assets = get_assets(assets_json).unwrap();
    download_assets(&client, assets, Path::new("/Volumes/tmpfs/minecorrosion_libraries/assets"));

    // TODO: JVM Arguments
    let jvm_arguments = parse_arguments_game_plus_jvm(&test_config, &environment_variables, &rules);

    // TODO: Game Arguments


    // let x = library_arguments.iter().fold(String::new(), |acc, x| {})


    panic!()
}

pub fn download_libraries(download_list: Vec<ArtifactStructure>, downloadroot: &str) {


}